use anyhow::{Result, Context};
use std::process::Command;
use crate::commands::build;
use crate::project;

pub fn handle(path: Option<String>) -> Result<()> {
    build::handle(path)?;
    
    let root = project::project_root()?;
    let generated_dir = root.join("generated");
    
    #[cfg(target_os = "windows")]
    let executable = generated_dir.join("target/release/jrust_app.exe");
    
    #[cfg(not(target_os = "windows"))]
    let executable = generated_dir.join("target/release/jrust_app");
    
    if !executable.exists() {
        anyhow::bail!("Executable not found: {:?}", executable);
    }
    
    println!();
    println!("🎯 Running program...");
    println!("─────────────────────────");
    
    let status = Command::new(&executable)
        .status()
        .context("Failed to execute program")?;
    
    println!("─────────────────────────");
    
    if !status.success() {
        anyhow::bail!("Program exited with error");
    }
    
    println!("✅ Program completed successfully!");
    Ok(())
}
