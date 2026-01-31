use serde::{Deserialize, Serialize};

/// Definition of a skill loaded from YAML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    /// Skill name (e.g. "commit").
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// The prompt template for this skill.
    pub prompt_template: String,
}

/// Registry for discovering and invoking skills.
pub trait SkillRegistry: Send + Sync {
    /// Lists all registered skill names.
    fn list(&self) -> Vec<String>;

    /// Retrieves a skill definition by name.
    fn get(&self, name: &str) -> Option<&SkillDefinition>;
}
