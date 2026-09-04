//! Agent manager for sub-agents

use crate::agent::SubAgent;

/// Agent manager error
#[derive(Debug, thiserror::Error)]
pub enum AgentManagerError {
    #[error("Agent not found: {0}")]
    NotFound(String),
}

/// Agent manager for managing sub-agents
pub struct AgentManager {
    agents: Vec<SubAgent>,
}

impl AgentManager {
    /// Create a new AgentManager
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
        }
    }

    /// Register a sub-agent
    pub fn register(&mut self, agent: SubAgent) {
        self.agents.push(agent);
    }

    /// Get all registered agents
    pub fn list(&self) -> &[SubAgent] {
        &self.agents
    }

    /// Find an agent by name
    pub fn find(&self, name: &str) -> Option<&SubAgent> {
        self.agents.iter().find(|a| a.name == name)
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}
