use anyhow::Result;
use clap::Parser;

/// Rust CLI template: Cargo workspace, clap, clippy, mise, lefthook, CI, release tarballs, Homebrew bump and a Zola docs site.
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
