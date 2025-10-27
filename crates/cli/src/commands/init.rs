use anyhow::Result;
use crate::project;

pub fn handle(project_name: String) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project_path = current_dir.join(&project_name);
    
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_name);
    }
    
    project::create_project_structure(&project_name, &project_path)?;
    
    println!("✨ Created new jRust project: {}", project_name);
    println!();
    println!("To get started:");
    println!("  cd {}", project_name);
    println!("  jrust run");
    
    Ok(())
}
