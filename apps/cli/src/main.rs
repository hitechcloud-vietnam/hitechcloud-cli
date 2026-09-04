//! HiTechCloud CLI - AI-native coding agent platform

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

mod commands;

/// HiTechCloud CLI - AI-native coding agent platform
#[derive(Parser)]
#[command(name = "hitechcloud")]
#[command(version = "0.1.0")]
#[command(about = "AI-native coding agent platform", long_about = None)]
struct Cli {
    /// Enable debug mode
    #[arg(long, global = true)]
    debug: bool,

    /// Log level
    #[arg(long, global = true, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive agent session
    #[command(alias = "i")]
    Interactive {
        /// Initial prompt to send to the agent
        #[arg(short, long)]
        prompt: Option<String>,

        /// Model to use
        #[arg(short, long)]
        model: Option<String>,

        /// Provider to use
        #[arg(short, long)]
        provider: Option<String>,
    },

    /// Run a single command and exit
    #[command(alias = "r")]
    Run {
        /// The prompt to execute
        prompt: String,

        /// Model to use
        #[arg(short, long)]
        model: Option<String>,

        /// Provider to use
        #[arg(short, long)]
        provider: Option<String>,
    },

    /// Initialize HiTechCloud configuration
    Init,

    /// Manage sessions
    Session {
        #[command(subcommand)]
        action: commands::session::SessionAction,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: commands::config::ConfigAction,
    },

    /// Manage skills
    Skills {
        #[command(subcommand)]
        action: commands::skills::SkillsAction,
    },

    /// Manage plugins
    Plugins {
        #[command(subcommand)]
        action: commands::plugins::PluginsAction,
    },

    /// Show version and system info
    #[command(alias = "v")]
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let filter = if cli.debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new(&cli.log_level)
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("HiTechCloud CLI v{}", env!("CARGO_PKG_VERSION"));

    // Initialize directories
    hitechcloud_core::HiTechCloudConfig::init_dirs()?;

    // Load configuration
    let config = hitechcloud_core::HiTechCloudConfig::load()?;

    match cli.command {
        Commands::Interactive { prompt, model, provider } => {
            commands::interactive::run(prompt, model, provider, &config).await?;
        }
        Commands::Run { prompt, model, provider } => {
            commands::run::run(prompt, model, provider, &config).await?;
        }
        Commands::Init => {
            commands::init::run(&config)?;
        }
        Commands::Session { action } => {
            commands::session::handle(action, &config).await?;
        }
        Commands::Config { action } => {
            commands::config::handle(action, &config).await?;
        }
        Commands::Skills { action } => {
            commands::skills::handle(action, &config).await?;
        }
        Commands::Plugins { action } => {
            commands::plugins::handle(action, &config).await?;
        }
        Commands::Version => {
            commands::version::show();
        }
    }

    Ok(())
}
