use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Top-level Kanata configuration, loaded from `~/.kanata/config.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanataConfig {
    /// Default model identifier.
    pub default_model: String,
    /// API keys keyed by provider name.
    pub api_keys: HashMap<String, String>,
    /// Trust level (1–4).
    pub trust_level: u8,
    /// Optional custom prompt directory.
    pub prompt_dir: Option<PathBuf>,
    /// Optional project memory path.
    pub memory_path: Option<PathBuf>,
}
