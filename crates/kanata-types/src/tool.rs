use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::KanataError;

/// Result of a tool execution.
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// The output content.
    pub content: String,
    /// Whether the tool encountered an error.
    pub is_error: bool,
}

/// JSON Schema definition for a tool, sent to the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// JSON Schema for the tool's input parameters.
    pub input_schema: serde_json::Value,
}

/// A tool that the agent can invoke.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the tool definition for the LLM.
    fn definition(&self) -> ToolDefinition;

    /// Executes the tool with the given JSON input.
    async fn execute(&self, input: serde_json::Value) -> Result<ToolResult, KanataError>;
}
