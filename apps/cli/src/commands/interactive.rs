//! Interactive mode command

use hitechcloud_core::HiTechCloudConfig;
use std::io::{self, Write};

pub async fn run(
    prompt: Option<String>,
    model: Option<String>,
    provider: Option<String>,
    config: &HiTechCloudConfig,
) -> anyhow::Result<()> {
    let provider_name = provider.as_deref().unwrap_or(
        config.default_provider.as_deref().unwrap_or("nube")
    );
    let model_name = model.as_deref().unwrap_or(
        config.default_model.as_deref().unwrap_or("gpt-4")
    );

    println!("🤖 HiTechCloud Interactive Mode");
    println!("  Provider: {}", provider_name);
    println!("  Model: {}", model_name);
    println!();
    println!("Type your message and press Enter. Type 'exit' or 'quit' to exit.");
    println!("Type '/help' for available commands.");
    println!();

    // Handle initial prompt if provided
    if let Some(initial_prompt) = prompt {
        println!("You: {}", initial_prompt);
        println!();
        // TODO: Send to agent and get response
        println!("Agent: I received your message. Processing...");
        println!();
    }

    // Main REPL loop
    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Handle special commands
        if input.starts_with('/') {
            match input {
                "/help" => {
                    println!();
                    println!("Available commands:");
                    println!("  /help     - Show this help message");
                    println!("  /model    - Show current model");
                    println!("  /provider - Show current provider");
                    println!("  /clear    - Clear conversation history");
                    println!("  /history  - Show conversation history");
                    println!("  /exit     - Exit the session");
                    println!();
                }
                "/model" => {
                    println!("Current model: {}", model_name);
                }
                "/provider" => {
                    println!("Current provider: {}", provider_name);
                }
                "/clear" => {
                    println!("Conversation history cleared.");
                }
                "/history" => {
                    println!("Conversation history:");
                    // TODO: Show actual history
                    println!("  (No history yet)");
                }
                "/exit" | "/quit" => {
                    println!("Goodbye! 👋");
                    break;
                }
                _ => {
                    println!("Unknown command: {}. Type '/help' for available commands.", input);
                }
            }
            continue;
        }

        // Handle exit commands
        if input == "exit" || input == "quit" {
            println!("Goodbye! 👋");
            break;
        }

        // TODO: Send to agent and get response
        println!();
        println!("Agent: I received your message: {}", input);
        println!("       (Agent integration not yet implemented)");
        println!();
    }

    Ok(())
}
