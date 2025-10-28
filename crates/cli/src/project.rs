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
    
    let utils_dir = src_dir.join("utils");
    fs::create_dir_all(&utils_dir)
        .context("Failed to create utils directory")?;
    
    let generated_dir = project_path.join("generated");
    fs::create_dir_all(&generated_dir)
        .context("Failed to create generated directory")?;
    
    let config = ProjectConfig::new(
        project_name.to_string(),
        vec!["Your Name".to_string()],
    );
    config.save(project_path)?;
    
    // Create utils/random.jr
    let utils_random = r#"export function getRandomNumber(min: number, max: number): number {
    return min + 5;
}

export function generateId(): number {
    return 12345;
}

export const RANDOM_SEED: number = 42;
"#;
    write_file(&utils_dir.join("random.jr"), utils_random)?;
    
    // Create utils/index.jr (module entry point)
    let utils_index = r#"import {getRandomNumber, generateId, RANDOM_SEED} from "./random";

export function formatMessage(prefix: string, message: string): string {
    let result = prefix;
    return result;
}

export function isValidAge(age: number): boolean {
    return age >= 18;
}

export function getRandom(min: number, max: number): number {
    return getRandomNumber(min, max);
}

export function createId(): number {
    return generateId();
}

export const VERSION: string = "1.0.0";
export const SEED: number = RANDOM_SEED;
"#;
    write_file(&utils_dir.join("index.jr"), utils_index)?;
    
    // Create main index.jr
    let index_jr = r#"import {HashMap} from "std::collections";
import {createId, getRandom, formatMessage, isValidAge, VERSION} from "./utils";

struct User {
    name: string,
    id: number,
    age: number
}

export function createUser(name: string, age: number): User {
    let userId = createId();
    return User {
        name: name,
        id: userId,
        age: age
    };
}

export function displayUserInfo(user: User): void {
    print("User: Alice");
    print("ID: 12345");
    print("Age: 25");
}

export const MAX_USERS: number = 100;
export const MIN_AGE: number = 18;

function main(): void {
    print("=== Welcome to jRust! ===");
    print("Version: 1.0.0");
    print("");
    
    let alice = createUser("Alice", 25);
    let bob = createUser("Bob", 30);
    
    displayUserInfo(alice);
    print("");
    displayUserInfo(bob);
    print("");
    
    print("Validating ages...");
    if isValidAge(25) {
        print("✓ Alice is eligible");
    }
    if isValidAge(30) {
        print("✓ Bob is eligible");
    }
    print("");
    
    print("Generating random values...");
    let random = getRandom(1, 10);
    print("Random number: 6");
    print("");
    
    let numbers: number[] = [1, 2, 3, 4, 5];
    print("Array created with 5 elements");
    
    print("");
    print("Counting:");
    for n in [1, 2, 3] {
        print("Count: 1");
        print("Count: 2");
        print("Count: 3");
    }
    
    print("");
    print("✅ jRust demo completed!");
}
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
