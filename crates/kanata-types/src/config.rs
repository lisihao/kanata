use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Top-level Kanata configuration, loaded from `~/.kanata/config.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanataConfig {
    /// Default model identifier.
    #[serde(default = "default_model")]
    pub default_model: String,
    /// API keys keyed by provider name.
    #[serde(default)]
    pub api_keys: HashMap<String, String>,
    /// Trust level (1–4).
    #[serde(default = "default_trust_level")]
    pub trust_level: u8,
    /// Optional custom prompt directory.
    #[serde(default)]
    pub prompt_dir: Option<PathBuf>,
    /// Optional project memory path.
    #[serde(default)]
    pub memory_path: Option<PathBuf>,
}

impl Default for KanataConfig {
    fn default() -> Self {
        Self {
            default_model: default_model(),
            api_keys: HashMap::new(),
            trust_level: default_trust_level(),
            prompt_dir: None,
            memory_path: None,
        }
    }
}

fn default_model() -> String {
    "claude-sonnet-4".to_string()
}

fn default_trust_level() -> u8 {
    1
}
