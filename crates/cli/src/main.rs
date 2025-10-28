mod project;
mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "jrust")]
#[command(about = "jRust - Simple language that compiles to Rust", long_about = None)]
#[command(version)]
#[command(author = "jRust Contributors")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new jRust project
    Init {
        /// Project name
        name: String,
    },
    
    /// Build a jRust program
    Build {
        /// Path to .jr file (optional, uses src/index.jr by default)
        path: Option<String>,
    },
    
    /// Run a jRust program
    Run {
        /// Path to .jr file (optional, uses src/index.jr by default)
        path: Option<String>,
    },
    
    /// Check syntax and types without generating code
    Check {
        /// Path to .jr file (optional, uses src/index.jr by default)
        path: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init { name } => commands::init::handle(name)?,
        Commands::Build { path } => commands::build::handle(path)?,
        Commands::Run { path } => commands::run::handle(path)?,
        Commands::Check { path } => commands::check::handle(path)?,
    }
    
    Ok(())
}

