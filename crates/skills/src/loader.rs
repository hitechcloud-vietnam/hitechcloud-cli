//! Skill loader

use crate::skill::Skill;
use std::path::Path;

/// Skill loader error
#[derive(Debug, thiserror::Error)]
pub enum SkillLoaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),
}

/// Skill loader for loading skills from files
pub struct SkillLoader;

impl SkillLoader {
    /// Load a skill from a SKILL.md file
    pub fn load_from_file(path: &Path) -> Result<Skill, SkillLoaderError> {
        let content = std::fs::read_to_string(path)?;

        // TODO: Parse frontmatter and content
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Skill::new(name, "Loaded from file", content))
    }

    /// Load all skills from a directory
    pub fn load_from_dir(dir: &Path) -> Result<Vec<Skill>, SkillLoaderError> {
        let mut skills = Vec::new();

        if dir.exists() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Ok(skill) = Self::load_from_file(&path) {
                        skills.push(skill);
                    }
                }
            }
        }

        Ok(skills)
    }
}
