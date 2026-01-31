//! Built-in skill definitions (commit, review, explain).

use kanata_types::skill::SkillDefinition;

/// Returns the built-in `commit` skill definition.
pub fn commit_skill() -> SkillDefinition {
    SkillDefinition {
        name: "commit".to_string(),
        description: "Analyze git diff and generate a conventional commit message.".to_string(),
        prompt_template: r#"Analyze the current git changes and generate a commit message.

Steps:
1. Run `git diff --staged` to see staged changes. If empty, run `git diff` for unstaged changes.
2. Analyze the nature of the changes (feat, fix, refactor, docs, test, chore, perf).
3. Draft a concise commit message in Conventional Commits format:
   `<type>(<scope>): <description>`
4. Show the proposed message and ask for confirmation before committing.

Rules:
- Keep the subject line under 72 characters.
- Use imperative mood ("add", not "added").
- If changes span multiple scopes, pick the most important one."#
            .to_string(),
    }
}

/// Returns the built-in `review` skill definition.
pub fn review_skill() -> SkillDefinition {
    SkillDefinition {
        name: "review".to_string(),
        description: "Review code changes and provide feedback.".to_string(),
        prompt_template: r#"Review the current code changes and provide feedback.

Steps:
1. Run `git diff` to see the changes.
2. For each changed file, analyze:
   - Correctness: Are there bugs or logic errors?
   - Style: Does the code follow project conventions?
   - Security: Are there potential vulnerabilities?
   - Performance: Are there obvious inefficiencies?
3. Provide actionable feedback with specific line references.

Output format:
- Start with a brief summary.
- List issues by severity (critical > warning > suggestion).
- End with an overall assessment."#
            .to_string(),
    }
}

/// Returns the built-in `explain` skill definition.
pub fn explain_skill() -> SkillDefinition {
    SkillDefinition {
        name: "explain".to_string(),
        description: "Explain how a file or code section works.".to_string(),
        prompt_template: r#"Explain the selected code or file to the user.

Steps:
1. Read the specified file or code section.
2. Identify the purpose, key components, and data flow.
3. Explain at an appropriate level of detail.

Guidelines:
- Start with a one-line summary of what the code does.
- Walk through the main logic flow.
- Highlight non-obvious patterns or design decisions.
- Note any potential issues or areas for improvement.
- Use clear, jargon-free language when possible."#
            .to_string(),
    }
}

/// Returns all built-in skill definitions.
pub fn all_builtin_skills() -> Vec<SkillDefinition> {
    vec![commit_skill(), review_skill(), explain_skill()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_builtin_skills() {
        let skills = all_builtin_skills();
        assert_eq!(skills.len(), 3);

        let names: Vec<&str> = skills.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"commit"));
        assert!(names.contains(&"review"));
        assert!(names.contains(&"explain"));
    }

    #[test]
    fn test_commit_skill_has_prompt() {
        let skill = commit_skill();
        assert!(!skill.prompt_template.is_empty());
        assert!(skill.prompt_template.contains("git diff"));
    }
}
