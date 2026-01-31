use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;

use crate::error::KanataError;
use crate::message::Message;
use crate::tool::ToolDefinition;

/// A streaming event emitted by the LLM.
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// Incremental text output.
    TextDelta(String),
    /// A tool invocation has started.
    ToolUseStart {
        /// Server-assigned tool-use ID.
        id: String,
        /// Tool name.
        name: String,
    },
    /// Incremental JSON fragment for the tool input.
    ToolUseDelta(String),
    /// The current tool invocation has ended.
    ToolUseEnd,
    /// The message is complete.
    MessageEnd {
        /// Token usage for this message.
        usage: TokenUsage,
    },
    /// An error occurred during streaming.
    Error(String),
}

/// Token usage statistics for a single LLM call.
#[derive(Debug, Clone, Default)]
pub struct TokenUsage {
    /// Number of input tokens.
    pub input_tokens: u32,
    /// Number of output tokens.
    pub output_tokens: u32,
    /// Tokens read from cache.
    pub cache_read_tokens: u32,
    /// Tokens written to cache.
    pub cache_write_tokens: u32,
    /// Model identifier.
    pub model: String,
    /// Estimated cost in USD.
    pub cost_usd: f64,
}

/// Unified LLM client interface.
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Sends a streaming chat request.
    async fn chat_stream(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        system: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent, KanataError>> + Send>>, KanataError>;

    /// Returns the model name.
    fn model_name(&self) -> &str;
}
