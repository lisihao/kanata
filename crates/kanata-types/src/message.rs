use serde::{Deserialize, Serialize};

/// Role of a message participant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// The human user.
    User,
    /// The AI assistant.
    Assistant,
}

/// Content within a user message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UserContent {
    /// Plain text content.
    #[serde(rename = "text")]
    Text {
        /// The text body.
        text: String,
    },
    /// A tool result returned to the model.
    #[serde(rename = "tool_result")]
    ToolResult {
        /// The tool use ID this result corresponds to.
        tool_use_id: String,
        /// The result content.
        content: String,
        /// Whether the tool execution errored.
        is_error: bool,
    },
}

/// A single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Who sent the message.
    pub role: Role,
    /// The message content.
    pub content: serde_json::Value,
}
