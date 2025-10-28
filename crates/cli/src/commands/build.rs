use anyhow::{Result, Context};
use jrust_transpiler_core::{Lexer, Parser, Codegen};
use crate::project;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::collections::HashMap;

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
    
    let root = project::project_root()?;
    let src_dir = root.join("src");
    
    println!("🔨 Building jRust project...");
    
    // Find all .jr files in the project
    let jr_files = find_all_jr_files(&src_dir)?;
    println!("📄 Found {} jRust file(s)", jr_files.len());
    
    let mut modules = HashMap::new();
    
    // Compile each .jr file
    for jr_file in &jr_files {
        let relative_path = jr_file.strip_prefix(&src_dir)
            .context("Failed to get relative path")?;
        
        let module_path = relative_path.with_extension("");
        let module_name = module_path.to_string_lossy().replace("\\", "/");
        
        println!("  � Compiling: {}", module_name);
        
        let source = project::read_source_file(jr_file)?;
        
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize()
            .map_err(|e| anyhow::anyhow!("Lexical analysis failed in {}: {}", module_name, e))?;
        
        let mut parser = Parser::new(tokens);
        let program = parser.parse()
            .map_err(|e| anyhow::anyhow!("Syntax parsing failed in {}: {}", module_name, e))?;
        
        // Use new_module() for non-main files to avoid wrapping in main()
        let mut codegen = if module_name == "index" {
            Codegen::new()
        } else {
            Codegen::new_module()
        };
        let rust_code = codegen.generate(&program);
        
        modules.insert(module_name, rust_code);
    }
    
    println!("✅ All files compiled successfully");
    
    let generated_dir = root.join("generated");
    
    // Write main.rs with module declarations
    let main_rs = generate_main_rs(&modules)?;
    project::write_file(&generated_dir.join("main.rs"), &main_rs)?;
    
    // Write module files
    for (module_name, rust_code) in &modules {
        if module_name != "index" {
            let module_file = if module_name.contains('/') {
                // For nested modules like "utils/random", create utils/random.rs
                let parts: Vec<&str> = module_name.split('/').collect();
                let dir = generated_dir.join(parts[0..parts.len()-1].join("/"));
                std::fs::create_dir_all(&dir)?;
                dir.join(format!("{}.rs", parts[parts.len()-1]))
            } else {
                generated_dir.join(format!("{}.rs", module_name))
            };
            project::write_file(&module_file, rust_code)?;
        }
    }
    
    // Generate mod.rs files for directories
    let mut directories = std::collections::HashSet::new();
    for module_name in modules.keys() {
        if module_name.contains('/') {
            let parts: Vec<&str> = module_name.split('/').collect();
            if parts.len() >= 2 {
                directories.insert(parts[0].to_string());
            }
        }
    }
    
    for dir_name in directories {
        let mod_rs_path = generated_dir.join(&dir_name).join("mod.rs");
        let mut mod_content = String::new();
        
        // Find all modules in this directory
        for module_name in modules.keys() {
            if let Some(stripped) = module_name.strip_prefix(&format!("{}/", dir_name)) {
                if !stripped.contains('/') {
                    mod_content.push_str(&format!("pub mod {};\n", stripped));
                    
                    // Re-export if the module is "index"
                    if stripped == "index" {
                        mod_content.push_str(&format!("pub use {}::*;\n", stripped));
                    }
                }
            }
        }
        
        project::write_file(&mod_rs_path, &mod_content)?;
    }
    
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
    let executable = if cfg!(windows) {
        generated_dir.join("target/release/jrust_app.exe")
    } else {
        generated_dir.join("target/release/jrust_app")
    };
    println!("📦 Executable: {:?}", executable);
    
    Ok(())
}

fn find_all_jr_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                files.extend(find_all_jr_files(&path)?);
            } else if path.extension().and_then(|s| s.to_str()) == Some("jr") {
                files.push(path);
            }
        }
    }
    
    Ok(files)
}

fn generate_main_rs(modules: &HashMap<String, String>) -> Result<String> {
    let mut output = String::new();
    
    // Add module declarations for non-index modules
    let mut module_names: Vec<&String> = modules.keys().collect();
    module_names.sort();
    
    for module_name in &module_names {
        if module_name.as_str() != "index" {
            if module_name.contains('/') {
                // For nested modules like "utils/random"
                let parts: Vec<&str> = module_name.split('/').collect();
                if parts.len() == 2 {
                    // Only declare the top-level module once
                    if !output.contains(&format!("mod {};", parts[0])) {
                        output.push_str(&format!("mod {};\n", parts[0]));
                    }
                }
            } else {
                output.push_str(&format!("mod {};\n", module_name));
            }
        }
    }
    
    output.push_str("\n");
    
    // Add the index module code (main code)
    if let Some(index_code) = modules.get("index") {
        output.push_str(index_code);
    }
    
    Ok(output)
}

fn generate_cargo_toml(project_root: &std::path::Path, generated_dir: &std::path::Path) -> Result<()> {
    let config = project::ProjectConfig::from_path(project_root)?;
    
    let cargo_toml = format!(
        r#"[package]
name = "jrust_app"
version = "{}"
edition = "{}"
authors = {}

[workspace]

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
