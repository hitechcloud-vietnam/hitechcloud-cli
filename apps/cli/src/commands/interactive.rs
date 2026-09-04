//! Interactive mode command

use hitechcloud_core::HiTechCloudConfig;

pub async fn run(
    prompt: Option<String>,
    model: Option<String>,
    provider: Option<String>,
    config: &HiTechCloudConfig,
) -> anyhow::Result<()> {
    tracing::info!("Starting interactive mode");

    // TODO: Implement interactive TUI
    println!("🤖 HiTechCloud Interactive Mode");
    println!("Type your message and press Enter. Type 'exit' or 'quit' to exit.");
    println!();

    if let Some(initial_prompt) = prompt {
        println!("> {}", initial_prompt);
        // TODO: Send to agent
    }

    // Simple REPL for now
    loop {
        print!("> ");
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" || input == "quit" {
            println!("Goodbye! 👋");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // TODO: Send to agent and get response
        println!("Agent: I received your message: {}", input);
    }

    Ok(())
}
