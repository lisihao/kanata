/// Skill registry and management for Kanata.
pub mod builtin;
pub mod loader;

use kanata_types::skill::{SkillDefinition, SkillRegistry};

/// In-memory skill registry loaded from YAML definitions.
pub struct InMemorySkillRegistry {
    skills: Vec<SkillDefinition>,
}

impl InMemorySkillRegistry {
    /// Creates an empty skill registry.
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    /// Creates a registry pre-loaded with all built-in skills.
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        for skill in builtin::all_builtin_skills() {
            registry.add(skill);
        }
        registry
    }

    /// Adds a skill definition to the registry.
    ///
    /// If a skill with the same name already exists, it is replaced.
    pub fn add(&mut self, skill: SkillDefinition) {
        self.skills.retain(|s| s.name != skill.name);
        self.skills.push(skill);
    }
}

impl Default for InMemorySkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillRegistry for InMemorySkillRegistry {
    fn list(&self) -> Vec<String> {
        self.skills.iter().map(|s| s.name.clone()).collect()
    }

    fn get(&self, name: &str) -> Option<&SkillDefinition> {
        self.skills.iter().find(|s| s.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_registry_add_and_lookup() {
        let mut registry = InMemorySkillRegistry::new();
        registry.add(SkillDefinition {
            name: "commit".to_string(),
            description: "Generate commit message".to_string(),
            prompt_template: "Analyze git diff and generate a commit message.".to_string(),
        });

        assert_eq!(registry.list(), vec!["commit"]);
        let skill = registry.get("commit").expect("skill exists");
        assert_eq!(skill.name, "commit");
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_with_builtins() {
        let registry = InMemorySkillRegistry::with_builtins();
        let names = registry.list();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"commit".to_string()));
        assert!(names.contains(&"review".to_string()));
        assert!(names.contains(&"explain".to_string()));
    }
}
