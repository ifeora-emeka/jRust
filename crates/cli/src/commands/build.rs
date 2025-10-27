use anyhow::{Result, Context};
use jrust_transpiler_core::{Lexer, Parser, Codegen};
use crate::project;
use std::path::PathBuf;
use std::process::Command;

pub fn handle(path: Option<String>) -> Result<()> {
    let file_path = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        let root = project::project_root()?;
        let src_dir = root.join("src");
        project::find_entry_point(&src_dir)?
    };
    
    if !file_path.exists() {
        anyhow::bail!("File not found: {:?}", file_path);
    }
    
    let source = project::read_source_file(&file_path)?;
    
    println!("🔨 Building: {:?}", file_path);
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()
        .map_err(|e| anyhow::anyhow!("Lexical analysis failed: {}", e))?;
    println!("✅ Lexical analysis passed");
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()
        .map_err(|e| anyhow::anyhow!("Syntax parsing failed: {}", e))?;
    println!("✅ Syntax parsing passed");
    
    let mut codegen = Codegen::new();
    let rust_code = codegen.generate(&program);
    println!("✅ Code generation passed");
    
    let root = project::project_root()?;
    let generated_dir = root.join("generated");
    let rust_file = generated_dir.join("main.rs");
    
    project::write_file(&rust_file, &rust_code)?;
    println!("✅ Generated Rust code");
    
    generate_cargo_toml(&root, &generated_dir)?;
    
    println!("🚀 Compiling with Rust...");
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&generated_dir)
        .status()
        .context("Failed to run cargo build")?;
    
    if !status.success() {
        anyhow::bail!("Cargo build failed");
    }
    
    println!("✅ Build completed successfully!");
    let executable = generated_dir.join("target/release/jrust_app.exe");
    println!("📦 Executable: {:?}", executable);
    
    Ok(())
}

fn generate_cargo_toml(project_root: &std::path::Path, generated_dir: &std::path::Path) -> Result<()> {
    let config = project::ProjectConfig::from_path(project_root)?;
    
    let cargo_toml = format!(
        r#"[package]
name = "jrust_app"
version = "{}"
edition = "{}"
authors = {}

[[bin]]
name = "jrust_app"
path = "main.rs"
"#,
        config.package.version,
        config.package.edition,
        format!("{:?}", config.package.authors)
    );
    
    project::write_file(&generated_dir.join("Cargo.toml"), &cargo_toml)?;
    Ok(())
}
