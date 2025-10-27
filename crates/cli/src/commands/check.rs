use anyhow::Result;
use jrust_transpiler_core::{Lexer, Parser};
use crate::project;

pub fn handle(path: Option<String>) -> Result<()> {
    let file_path = if let Some(p) = path {
        std::path::PathBuf::from(p)
    } else {
        let root = project::project_root()?;
        let src_dir = root.join("src");
        project::find_entry_point(&src_dir)?
    };
    
    if !file_path.exists() {
        anyhow::bail!("File not found: {:?}", file_path);
    }
    
    let source = project::read_source_file(&file_path)?;
    
    println!("📋 Checking: {:?}", file_path);
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()
        .map_err(|e| anyhow::anyhow!(e))?;
    println!("✅ Lexical analysis passed");
    
    let mut parser = Parser::new(tokens);
    let _program = parser.parse()
        .map_err(|e| anyhow::anyhow!(e))?;
    println!("✅ Syntax parsing passed");
    
    println!("✅ All checks passed!");
    Ok(())
}
