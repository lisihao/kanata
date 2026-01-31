//! Agent main loop — orchestrates conversation between LLM and tools.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use futures::stream;
use futures::{Stream, StreamExt};
use tokio::sync::Mutex;

use kanata_types::error::KanataError;
use kanata_types::llm::{LLMClient, StreamEvent, TokenUsage};
use kanata_types::message::{Message, Role};
use kanata_types::session::{AgentEvent, AgentSession, SessionTokenStats};
use kanata_types::tool::{Tool, ToolDefinition};

/// The core agent that drives the conversation loop.
pub struct Agent {
    llm: Box<dyn LLMClient>,
    tools: Vec<Box<dyn Tool>>,
    messages: Arc<Mutex<Vec<Message>>>,
    system_prompt: String,
    stats: Arc<Mutex<SessionTokenStats>>,
}

impl Agent {
    /// Creates a new agent with the given LLM client and tools.
    pub fn new(
        llm: Box<dyn LLMClient>,
        tools: Vec<Box<dyn Tool>>,
        system_prompt: impl Into<String>,
    ) -> Self {
        let model = llm.model_name().to_string();
        Self {
            llm,
            tools,
            messages: Arc::new(Mutex::new(Vec::new())),
            system_prompt: system_prompt.into(),
            stats: Arc::new(Mutex::new(SessionTokenStats {
                model,
                ..SessionTokenStats::default()
            })),
        }
    }

    /// Returns tool definitions for all registered tools.
    fn tool_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.iter().map(|t| t.definition()).collect()
    }

    /// Finds a tool by name.
    fn find_tool(&self, name: &str) -> Option<&dyn Tool> {
        self.tools
            .iter()
            .find(|t| t.definition().name == name)
            .map(AsRef::as_ref)
    }

    /// Accumulates token usage into session stats.
    async fn accumulate_usage(&self, usage: &TokenUsage) {
        let mut stats = self.stats.lock().await;
        stats.total_input_tokens += usage.input_tokens;
        stats.total_output_tokens += usage.output_tokens;
        stats.total_cost_usd += usage.cost_usd;
    }

    /// Runs one turn: sends messages to LLM, collects response, handles tool
    /// calls. Returns a list of `AgentEvent`s for the CLI to render.
    fn run_turn(&self) -> Pin<Box<dyn Future<Output = Result<Vec<AgentEvent>, KanataError>> + Send + '_>> {
        Box::pin(self.run_turn_inner())
    }

    async fn run_turn_inner(&self) -> Result<Vec<AgentEvent>, KanataError> {
        let mut events = vec![AgentEvent::Thinking];
        let tool_defs = self.tool_definitions();
        let messages = self.messages.lock().await.clone();

        let mut llm_stream = self
            .llm
            .chat_stream(&messages, &tool_defs, &self.system_prompt)
            .await?;

        let mut text_accum = String::new();
        let mut tool_name = String::new();
        let mut tool_id = String::new();
        let mut tool_input_json = String::new();
        let mut pending_tool_calls: Vec<(String, String, String)> = Vec::new();
        let mut final_usage: Option<TokenUsage> = None;

        while let Some(event_result) = llm_stream.next().await {
            match event_result? {
                StreamEvent::TextDelta(text) => {
                    events.push(AgentEvent::TextDelta(text.clone()));
                    text_accum.push_str(&text);
                }
                StreamEvent::ToolUseStart { id, name } => {
                    tool_id = id;
                    tool_name = name;
                    tool_input_json.clear();
                }
                StreamEvent::ToolUseDelta(delta) => {
                    tool_input_json.push_str(&delta);
                }
                StreamEvent::ToolUseEnd => {
                    pending_tool_calls.push((
                        tool_id.clone(),
                        tool_name.clone(),
                        tool_input_json.clone(),
                    ));
                }
                StreamEvent::MessageEnd { usage } => {
                    final_usage = Some(usage);
                }
                StreamEvent::Error(e) => {
                    events.push(AgentEvent::Error(e));
                }
            }
        }

        // Append assistant message to history.
        if !text_accum.is_empty() {
            let mut msgs = self.messages.lock().await;
            msgs.push(Message {
                role: Role::Assistant,
                content: serde_json::Value::String(text_accum),
            });
        }

        if let Some(usage) = &final_usage {
            self.accumulate_usage(usage).await;
        }

        // Execute any tool calls.
        if !pending_tool_calls.is_empty() {
            for (call_id, name, input_json) in &pending_tool_calls {
                let input_preview =
                    truncate_str(input_json, 100).to_string();
                events.push(AgentEvent::ToolStart {
                    name: name.clone(),
                    input_preview,
                });

                let input: serde_json::Value =
                    serde_json::from_str(input_json).unwrap_or_default();

                let result = if let Some(tool) = self.find_tool(name) {
                    match tool.execute(input).await {
                        Ok(r) => r,
                        Err(e) => kanata_types::ToolResult {
                            content: e.to_string(),
                            is_error: true,
                        },
                    }
                } else {
                    kanata_types::ToolResult {
                        content: format!("Unknown tool: {name}"),
                        is_error: true,
                    }
                };

                let result_preview =
                    truncate_str(&result.content, 200).to_string();
                events.push(AgentEvent::ToolEnd {
                    name: name.clone(),
                    result_preview,
                });

                // Add tool result to message history for next turn.
                let mut msgs = self.messages.lock().await;
                msgs.push(Message {
                    role: Role::User,
                    content: serde_json::json!([{
                        "type": "tool_result",
                        "tool_use_id": call_id,
                        "content": result.content,
                        "is_error": result.is_error,
                    }]),
                });
            }

            // Recurse: send tool results back to LLM for a follow-up.
            let follow_up = self.run_turn().await?;  // boxed recursion
            events.extend(follow_up);
        }

        Ok(events)
    }
}

#[async_trait]
impl AgentSession for Agent {
    async fn send_message(
        &self,
        content: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = AgentEvent> + Send>>, KanataError> {
        // Add user message to history.
        {
            let mut msgs = self.messages.lock().await;
            msgs.push(Message {
                role: Role::User,
                content: serde_json::Value::String(content.to_string()),
            });
        }

        // Increment turn count.
        {
            let mut stats = self.stats.lock().await;
            stats.turns += 1;
        }

        let events = self.run_turn().await?;

        // Append Done event with current stats.
        let stats = self.stats.lock().await.clone();
        let mut all_events = events;
        all_events.push(AgentEvent::Done { usage: stats });

        Ok(Box::pin(stream::iter(all_events)))
    }

    fn stats(&self) -> SessionTokenStats {
        // We can't await here, so use try_lock.
        self.stats
            .try_lock()
            .map(|s| s.clone())
            .unwrap_or_default()
    }

    async fn execute_command(&self, cmd: &str, _args: &str) -> Result<String, KanataError> {
        match cmd {
            "/help" => Ok("Available commands: /help, /model, /cost, /clear".to_string()),
            _ => Ok(format!("Unknown command: {cmd}")),
        }
    }
}

/// Truncates a string to at most `max_len` characters, appending `...` if
/// truncated.
fn truncate_str(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use super::*;

    /// A simple mock LLM for agent tests (no kanata-model dependency needed).
    struct SimpleMockLLM;

    #[async_trait]
    impl LLMClient for SimpleMockLLM {
        async fn chat_stream(
            &self,
            _messages: &[Message],
            _tools: &[ToolDefinition],
            _system: &str,
        ) -> Result<
            Pin<Box<dyn Stream<Item = Result<StreamEvent, KanataError>> + Send>>,
            KanataError,
        > {
            let events = vec![
                Ok(StreamEvent::TextDelta("Hello!".to_string())),
                Ok(StreamEvent::MessageEnd {
                    usage: TokenUsage::default(),
                }),
            ];
            Ok(Box::pin(stream::iter(events)))
        }

        fn model_name(&self) -> &str {
            "test-mock"
        }
    }

    #[tokio::test]
    async fn test_agent_send_message_returns_events() {
        let agent = Agent::new(Box::new(SimpleMockLLM), vec![], "You are helpful.");
        let stream = agent.send_message("hi").await.expect("send");
        let events: Vec<_> = stream.collect().await;

        assert!(events.iter().any(|e| matches!(e, AgentEvent::Thinking)));
        assert!(events.iter().any(|e| matches!(e, AgentEvent::TextDelta(_))));
        assert!(events.iter().any(|e| matches!(e, AgentEvent::Done { .. })));
    }

    #[tokio::test]
    async fn test_agent_stats_increments_turns() {
        let agent = Agent::new(Box::new(SimpleMockLLM), vec![], "");
        let _ = agent.send_message("first").await.expect("send");
        assert_eq!(agent.stats().turns, 1);
        let _ = agent.send_message("second").await.expect("send");
        assert_eq!(agent.stats().turns, 2);
    }

    #[tokio::test]
    async fn test_agent_execute_command_help() {
        let agent = Agent::new(Box::new(SimpleMockLLM), vec![], "");
        let result = agent.execute_command("/help", "").await.expect("cmd");
        assert!(result.contains("Available commands"));
    }
}
