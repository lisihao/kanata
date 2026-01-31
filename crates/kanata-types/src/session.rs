use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;

use crate::error::KanataError;

/// An event emitted by the agent session, consumed by the CLI layer.
#[derive(Debug, Clone)]
pub enum AgentEvent {
    /// The agent is thinking.
    Thinking,
    /// Incremental text output.
    TextDelta(String),
    /// A tool invocation has started.
    ToolStart {
        /// Tool name.
        name: String,
        /// Preview of the tool input.
        input_preview: String,
    },
    /// A tool invocation has completed.
    ToolEnd {
        /// Tool name.
        name: String,
        /// Preview of the tool result.
        result_preview: String,
    },
    /// The session turn is complete.
    Done {
        /// Cumulative token statistics.
        usage: SessionTokenStats,
    },
    /// An error occurred.
    Error(String),
}

/// Cumulative token statistics for a session.
#[derive(Debug, Clone, Default)]
pub struct SessionTokenStats {
    /// Total input tokens across all turns.
    pub total_input_tokens: u32,
    /// Total output tokens across all turns.
    pub total_output_tokens: u32,
    /// Total estimated cost in USD.
    pub total_cost_usd: f64,
    /// Number of conversation turns.
    pub turns: u32,
    /// Current model name.
    pub model: String,
}

/// The agent session interface used by the CLI layer.
#[async_trait]
pub trait AgentSession: Send + Sync {
    /// Sends a user message and returns a stream of agent events.
    async fn send_message(
        &self,
        content: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = AgentEvent> + Send>>, KanataError>;

    /// Returns current session statistics.
    fn stats(&self) -> SessionTokenStats;

    /// Executes a slash command.
    async fn execute_command(&self, cmd: &str, args: &str) -> Result<String, KanataError>;
}
