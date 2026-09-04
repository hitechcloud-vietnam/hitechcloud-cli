//! Init command - initialize HiTechCloud configuration

use hitechcloud_core::HiTechCloudConfig;

pub fn run(config: &HiTechCloudConfig) -> anyhow::Result<()> {
    println!("🚀 Initializing HiTechCloud CLI...");
    println!();

    // Create directories
    HiTechCloudConfig::init_dirs()?;
    println!("✅ Created ~/.hitechcloud/ directory structure");

    // Save default config
    config.save()?;
    println!("✅ Created ~/.hitechcloud/config.toml");

    // Show configuration
    println!();
    println!("📋 Configuration:");
    println!("  Default provider: {}", config.default_provider.as_deref().unwrap_or("none"));
    println!("  Default model: {}", config.default_model.as_deref().unwrap_or("none"));
    println!("  Approval mode: {}", config.agent.approval_mode);
    println!("  Session dir: {}", config.session.data_dir.display());
    println!();

    // Check for API keys
    println!("🔑 API Keys:");
    for provider in &config.providers {
        let has_key = config.get_api_key(&provider.name).is_some();
        let status = if has_key { "✅" } else { "❌" };
        println!("  {} {}: {}", status, provider.name, if has_key { "configured" } else { "not set" });
    }

    println!();
    println!("💡 Set API keys via environment variables:");
    println!("  export NUBE_API_KEY=your_key_here");
    println!("  export OPENAI_API_KEY=your_key_here");
    println!("  export ANTHROPIC_API_KEY=your_key_here");
    println!();
    println!("✨ Initialization complete! Run 'hitechcloud interactive' to start.");

    Ok(())
}
