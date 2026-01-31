//! Context assembler — manages message history and token window.

use kanata_types::message::Message;

/// Manages the conversation context window.
///
/// Tracks messages and ensures the total token count stays within the model's
/// context limit by truncating older messages when necessary.
pub struct ContextAssembler {
    /// Maximum tokens allowed in the context window.
    max_tokens: u32,
    /// Approximate tokens per message (rough heuristic: 4 chars ≈ 1 token).
    chars_per_token: u32,
}

impl ContextAssembler {
    /// Creates a new context assembler with the given token limit.
    pub fn new(max_tokens: u32) -> Self {
        Self {
            max_tokens,
            chars_per_token: 4,
        }
    }

    /// Estimates the token count for a message.
    pub fn estimate_tokens(&self, message: &Message) -> u32 {
        let chars = message.content.to_string().len() as u32;
        chars / self.chars_per_token + 1
    }

    /// Estimates total tokens for a list of messages.
    pub fn estimate_total_tokens(&self, messages: &[Message]) -> u32 {
        messages.iter().map(|m| self.estimate_tokens(m)).sum()
    }

    /// Truncates the message list to fit within the token budget.
    ///
    /// Keeps the system-level first message (if present) and the most recent
    /// messages, dropping older ones from the middle.
    pub fn truncate_to_fit(&self, messages: &mut Vec<Message>) {
        while messages.len() > 2 && self.estimate_total_tokens(messages) > self.max_tokens {
            // Remove the second message (preserve first which is often system context,
            // and always preserve latest).
            messages.remove(1);
        }
    }

    /// Returns the configured maximum token count.
    pub fn max_tokens(&self) -> u32 {
        self.max_tokens
    }
}

impl Default for ContextAssembler {
    fn default() -> Self {
        Self::new(100_000)
    }
}

#[cfg(test)]
mod tests {
    use kanata_types::message::Role;

    use super::*;

    fn make_msg(text: &str) -> Message {
        Message {
            role: Role::User,
            content: serde_json::Value::String(text.to_string()),
        }
    }

    #[test]
    fn test_estimate_tokens() {
        let ctx = ContextAssembler::new(1000);
        let msg = make_msg("hello world!"); // 12 chars => 4 tokens
        assert!(ctx.estimate_tokens(&msg) > 0);
    }

    #[test]
    fn test_truncate_removes_old_messages() {
        let ctx = ContextAssembler::new(20); // Very small budget
        let mut messages = vec![
            make_msg("first message with some content"),
            make_msg("second message with some content"),
            make_msg("third message with some content"),
            make_msg("latest message"),
        ];
        ctx.truncate_to_fit(&mut messages);
        // Should have removed some middle messages
        assert!(messages.len() < 4);
        // First and last should be preserved
        assert!(messages.first().unwrap().content.to_string().contains("first"));
        assert!(messages.last().unwrap().content.to_string().contains("latest"));
    }

    #[test]
    fn test_no_truncation_when_under_limit() {
        let ctx = ContextAssembler::new(100_000);
        let mut messages = vec![make_msg("short"), make_msg("msg")];
        ctx.truncate_to_fit(&mut messages);
        assert_eq!(messages.len(), 2);
    }
}
