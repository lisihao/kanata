/// Unified error type for the Kanata project.
#[derive(Debug, thiserror::Error)]
pub enum KanataError {
    /// File not found at the specified path.
    #[error("File not found: {path}")]
    FileNotFound {
        /// The path that was not found.
        path: String,
    },

    /// A tool failed to execute.
    #[error("Tool execution failed: {tool_name}: {reason}")]
    ToolError {
        /// Name of the tool that failed.
        tool_name: String,
        /// Reason for the failure.
        reason: String,
    },

    /// The model API returned an error.
    #[error("Model API error: {status} {message}")]
    ModelError {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
    },

    /// The API returned a rate limit response.
    #[error("Rate limited, retry after {retry_after_secs}s")]
    RateLimited {
        /// Seconds to wait before retrying.
        retry_after_secs: u64,
    },

    /// The context window has been exceeded.
    #[error("Context window exceeded: {used}/{limit} tokens")]
    ContextOverflow {
        /// Tokens used so far.
        used: u32,
        /// Maximum tokens allowed.
        limit: u32,
    },

    /// A configuration error occurred.
    #[error("Configuration error: {0}")]
    Config(String),

    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// An HTTP error occurred.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// A JSON serialization/deserialization error occurred.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
