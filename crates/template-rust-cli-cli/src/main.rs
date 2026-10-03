use anyhow::Result;
use clap::Parser;

/// Rust CLI template with CI, releases and docs.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Who to greet.
    #[arg(default_value = "world")]
    name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("{}", template_rust_cli_core::greet(&cli.name)?);
    Ok(())
}
