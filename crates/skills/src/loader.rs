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

        // Parse frontmatter and content
        let (name, description, instructions) = Self::parse_skill_content(&content, path);

        Ok(Skill::new(name, description, instructions))
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

    /// Load skills from the default locations
    pub fn load_default_skills() -> Result<Vec<Skill>, SkillLoaderError> {
        let mut skills = Vec::new();

        // Load from ~/.hitechcloud/skills/
        let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let global_skills_dir = home.join(".hitechcloud").join("skills");
        skills.extend(Self::load_from_dir(&global_skills_dir)?);

        // Load from .hitechcloud/skills/ in current directory
        let local_skills_dir = std::path::PathBuf::from(".hitechcloud").join("skills");
        skills.extend(Self::load_from_dir(&local_skills_dir)?);

        Ok(skills)
    }

    /// Parse skill content from markdown
    fn parse_skill_content(content: &str, path: &Path) -> (String, String, String) {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut description = String::new();
        let mut instructions = String::new();
        let mut in_frontmatter = false;
        let mut frontmatter_ended = false;

        for line in content.lines() {
            if line.trim() == "---" {
                if !in_frontmatter {
                    in_frontmatter = true;
                    continue;
                } else {
                    in_frontmatter = false;
                    frontmatter_ended = true;
                    continue;
                }
            }

            if in_frontmatter {
                // Parse frontmatter
                if let Some(desc) = line.strip_prefix("description:") {
                    description = desc.trim().to_string();
                }
            } else if frontmatter_ended {
                instructions.push_str(line);
                instructions.push('\n');
            }
        }

        if description.is_empty() {
            description = format!("Skill loaded from {}", path.display());
        }

        (name, description, instructions)
    }
}
