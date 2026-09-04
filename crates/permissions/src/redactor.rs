//! Secret redactor for sensitive information

use regex::Regex;

/// Secret redactor for masking sensitive information
pub struct SecretRedactor {
    patterns: Vec<Regex>,
}

impl SecretRedactor {
    /// Create a new SecretRedactor
    pub fn new() -> Self {
        let patterns = vec![
            // API keys
            Regex::new(r#"(?i)(api[_-]?key|apikey)\s*[:=]\s*['""]?([a-zA-Z0-9_\-]{20,})['""]?"#).unwrap(),
            // AWS keys
            Regex::new(r#"(?i)(aws[_-]?access[_-]?key[_-]?id|aws[_-]?secret[_-]?access[_-]?key)\s*[:=]\s*['""]?([a-zA-Z0-9/+=]{20,})['""]?"#).unwrap(),
            // Generic secrets
            Regex::new(r#"(?i)(secret|password|token|credential)\s*[:=]\s*['""]?([^\s'""]{8,})['""]?"#).unwrap(),
            // Private keys
            Regex::new(r"-----BEGIN (RSA |EC |DSA )?PRIVATE KEY-----").unwrap(),
        ];

        Self { patterns }
    }

    /// Redact secrets from text
    pub fn redact(&self, text: &str) -> String {
        let mut result = text.to_string();

        for pattern in &self.patterns {
            result = pattern.replace_all(&result, "[REDACTED]").to_string();
        }

        result
    }

    /// Check if text contains secrets
    pub fn contains_secrets(&self, text: &str) -> bool {
        self.patterns.iter().any(|p| p.is_match(text))
    }
}

impl Default for SecretRedactor {
    fn default() -> Self {
        Self::new()
    }
}
