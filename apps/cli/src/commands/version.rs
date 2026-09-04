//! Version command

pub fn show() {
    println!("🤖 HiTechCloud CLI");
    println!("  Version:    {}", env!("CARGO_PKG_VERSION"));
    println!("  Build:      {}", env!("CARGO_PKG_NAME"));
    println!("  Rust:       {}", rustc_version());
    println!("  OS:         {}", std::env::consts::OS);
    println!("  Arch:       {}", std::env::consts::ARCH);
    println!();
    println!("  Repository: https://github.com/hitechcloud-vietnam/hitechcloud-cli");
    println!("  Homepage:   https://cli.hitechcloud.vn");
    println!("  MCP:        https://mcp-cli.hitechcloud.vn");
}

fn rustc_version() -> String {
    // Try to get rustc version from environment
    option_env!("RUSTC_VERSION")
        .unwrap_or("unknown")
        .to_string()
}
