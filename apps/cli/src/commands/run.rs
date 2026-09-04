//! Run command - execute a single prompt

use hitechcloud_core::HiTechCloudConfig;

pub async fn run(
    prompt: String,
    model: Option<String>,
    provider: Option<String>,
    config: &HiTechCloudConfig,
) -> anyhow::Result<()> {
    tracing::info!("Running single command: {}", prompt);

    // TODO: Implement agent execution
    println!("🤖 HiTechCloud Agent");
    println!("Prompt: {}", prompt);
    println!();
    println!("Processing... (not yet implemented)");

    Ok(())
}
