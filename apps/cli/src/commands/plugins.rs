//! Plugins management commands

use clap::Subcommand;
use hitechcloud_core::HiTechCloudConfig;

#[derive(Subcommand)]
pub enum PluginsAction {
    /// List installed plugins
    #[command(alias = "ls")]
    List,

    /// Install a plugin
    Install {
        /// Plugin name
        name: String,
    },

    /// Uninstall a plugin
    #[command(alias = "rm")]
    Uninstall {
        /// Plugin name
        name: String,
    },

    /// Show plugin details
    Show {
        /// Plugin name
        name: String,
    },
}

pub async fn handle(action: PluginsAction, config: &HiTechCloudConfig) -> anyhow::Result<()> {
    match action {
        PluginsAction::List => {
            println!("🔌 Installed Plugins:");
            // TODO: Implement plugins listing
            println!("  No plugins installed");
        }
        PluginsAction::Install { name } => {
            println!("Installing plugin: {}", name);
            // TODO: Implement plugin installation
        }
        PluginsAction::Uninstall { name } => {
            println!("Uninstalling plugin: {}", name);
            // TODO: Implement plugin uninstallation
        }
        PluginsAction::Show { name } => {
            println!("Showing plugin: {}", name);
            // TODO: Implement plugin show
        }
    }

    Ok(())
}
