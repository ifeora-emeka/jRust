use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub package: PackageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
}

impl ProjectConfig {
    pub fn new(name: String, authors: Vec<String>) -> Self {
        Self {
            package: PackageConfig {
                name,
                version: "0.0.1".to_string(),
                edition: "2021".to_string(),
                authors,
                description: Some("A jRust project".to_string()),
            },
        }
    }

    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).context("Failed to serialize project config to TOML")
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let config_path = path.join("jrust.toml");
        let content = fs::read_to_string(&config_path)
            .context("Failed to read jrust.toml")?;
        toml::from_str(&content)
            .context("Failed to parse jrust.toml")
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let config_path = path.join("jrust.toml");
        let content = self.to_toml()?;
        fs::write(&config_path, content)
            .context("Failed to write jrust.toml")?;
        Ok(())
    }
}

pub fn project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory")?;
    
    if current_dir.join("jrust.toml").exists() {
        return Ok(current_dir);
    }
    
    Err(anyhow::anyhow!(
        "Not a jRust project. Run 'jrust init <name>' to create a new project."
    ))
}

pub fn read_source_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .context(format!("Failed to read source file: {:?}", path))
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create directory: {:?}", parent))?;
    }
    fs::write(path, content)
        .context(format!("Failed to write file: {:?}", path))?;
    Ok(())
}

pub fn create_project_structure(project_name: &str, project_path: &Path) -> Result<()> {
    fs::create_dir_all(project_path)
        .context(format!("Failed to create project directory: {:?}", project_path))?;
    
    let src_dir = project_path.join("src");
    fs::create_dir_all(&src_dir)
        .context("Failed to create src directory")?;
    
    let generated_dir = project_path.join("generated");
    fs::create_dir_all(&generated_dir)
        .context("Failed to create generated directory")?;
    
    let config = ProjectConfig::new(
        project_name.to_string(),
        vec!["Your Name".to_string()],
    );
    config.save(project_path)?;
    
    let index_jr = r#"let name: string = "jRust";
print("Welcome to ");
print(name);

let numbers: number[] = [10, 20, 30, 40, 50];

let first: number = numbers[0];
print("First element: ");
print(first);

let len: number = numbers.length;
print("Array length: ");
print(len);

const THRESHOLD: number = 25;

if first >= THRESHOLD {
    print("First number is above threshold");
} else {
    print("First number is below threshold");
}

print("Numbers greater than 15:");
let items: number[] = [15, 20, 25, 30];

for item in items {
    if item > 15 {
        print(item);
    }
}

print("Loop with break and continue:");
for n in [1, 2, 3, 4, 5] {
    if n == 2 {
        continue;
    }
    if n == 4 {
        break;
    }
    print(n);
}

let message: any = "jRust supports flexible types!";
print(message);
"#;
    write_file(&src_dir.join("index.jr"), index_jr)?;
    
    let gitignore = r#"/target/
/generated/
*.exe
*.dll
*.so
*.dylib
.DS_Store
*.swp
*.swo
*~
"#;
    write_file(&project_path.join(".gitignore"), gitignore)?;
    
    Ok(())
}

pub fn find_entry_point(src_dir: &Path) -> Result<PathBuf> {
    let index_jr = src_dir.join("index.jr");
    if index_jr.exists() {
        return Ok(index_jr);
    }
    
    Err(anyhow::anyhow!(
        "No entry point found. Create src/index.jr to get started."
    ))
}
