//! Skill definition

use serde::{Deserialize, Serialize};

/// Skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub instructions: String,
    pub tools: Vec<String>,
}

impl Skill {
    /// Create a new Skill
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        instructions: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: "0.1.0".to_string(),
            author: None,
            instructions: instructions.into(),
            tools: vec![],
        }
    }
}
