//! Plugin definition

use serde::{Deserialize, Serialize};

/// Plugin definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub entry_point: String,
    pub dependencies: Vec<String>,
}

impl Plugin {
    /// Create a new Plugin
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            author: None,
            entry_point: "index.js".to_string(),
            dependencies: vec![],
        }
    }
}
