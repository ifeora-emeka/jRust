use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn create_test_project(name: &str) -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(name)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    temp_dir
}

#[test]
fn test_jrust_help() {
    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("jRust"))
        .stdout(predicate::str::contains("Initialize"))
        .stdout(predicate::str::contains("Build"))
        .stdout(predicate::str::contains("Run"));
}

#[test]
fn test_jrust_version() {
    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn test_init_creates_project_structure() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "test-project";

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(project_name)
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created new jRust project"));

    let project_path = temp_dir.path().join(project_name);

    assert!(
        project_path.exists(),
        "Project directory should be created"
    );
    assert!(
        project_path.join("Cargo.toml").exists(),
        "Cargo.toml should be created"
    );
    assert!(
        project_path.join("src").exists(),
        "src directory should be created"
    );
    assert!(
        project_path.join("src/index.jr").exists(),
        "src/index.jr should be created"
    );
    assert!(
        project_path.join(".gitignore").exists(),
        ".gitignore should be created"
    );
    assert!(
        project_path.join("generated").exists(),
        "generated directory should be created"
    );
}

#[test]
fn test_init_creates_valid_cargo_toml() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "config-test";

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(project_name)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let config_path = temp_dir.path().join(project_name).join("Cargo.toml");
    let content = fs::read_to_string(&config_path).expect("Failed to read Cargo.toml");

    assert!(
        content.contains("[package]"),
        "Config should have [package] section"
    );
    assert!(
        content.contains(&format!("name = '{}'", project_name)),
        "Config should have project name"
    );
    assert!(
        content.contains("version = '0.0.1'"),
        "Config should have version"
    );
}

#[test]
fn test_init_creates_index_jr_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "template-test";

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(project_name)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let project_path = temp_dir.path().join(project_name);
    let index_jr_path = project_path.join("src/index.jr");
    let utils_index_path = project_path.join("src/utils/index.jr");
    let utils_random_path = project_path.join("src/utils/random.jr");

    // Check files exist
    assert!(
        index_jr_path.exists(),
        "index.jr should exist"
    );
    assert!(
        utils_index_path.exists(),
        "utils/index.jr should exist"
    );
    assert!(
        utils_random_path.exists(),
        "utils/random.jr should exist"
    );

    // Check main index.jr content
    let content = fs::read_to_string(&index_jr_path).expect("Failed to read index.jr");
    assert!(
        content.contains("import {createId, getRandom, RANDOM_SEED} from \"./utils\""),
        "Template should import from utils"
    );
    assert!(
        content.contains("const VERSION"),
        "Template should have VERSION constant"
    );
    assert!(
        content.contains("const MAX_COUNT"),
        "Template should have MAX_COUNT constant"
    );
    assert!(
        content.contains("function greet"),
        "Template should have greet function"
    );
    assert!(
        content.contains("function calculate"),
        "Template should have calculate function"
    );
    assert!(
        content.contains("let numbers: number[]"),
        "Template should have number array"
    );
    assert!(
        content.contains("function main()"),
        "Template should have main function"
    );

    // Check utils/index.jr content
    let utils_content = fs::read_to_string(&utils_index_path).expect("Failed to read utils/index.jr");
    assert!(
        utils_content.contains("import {randomInRange, generateUniqueId, SEED_VALUE} from \"./random\""),
        "utils/index.jr should import from random"
    );
    assert!(
        utils_content.contains("export function getRandom"),
        "utils/index.jr should have getRandom wrapper"
    );
    assert!(
        utils_content.contains("export function createId"),
        "utils/index.jr should have createId wrapper"
    );

    // Check utils/random.jr content
    let random_content = fs::read_to_string(&utils_random_path).expect("Failed to read utils/random.jr");
    assert!(
        random_content.contains("export function randomInRange"),
        "utils/random.jr should have randomInRange"
    );
    assert!(
        random_content.contains("export const SEED_VALUE"),
        "utils/random.jr should have SEED_VALUE"
    );
}

#[test]
fn test_check_validates_syntax() {
    let temp_dir = create_test_project("check-test");
    let project_path = temp_dir.path().join("check-test");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("check")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Lexical analysis passed"))
        .stdout(predicate::str::contains("Syntax parsing passed"));
}

#[test]
fn test_check_fails_on_invalid_syntax() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "invalid-syntax-test";

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(project_name)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let project_path = temp_dir.path().join(project_name);
    let index_jr_path = project_path.join("src/index.jr");

    fs::write(&index_jr_path, "function broken(").expect("Failed to write invalid syntax");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("check")
        .current_dir(&project_path)
        .assert()
        .failure();
}

#[test]
fn test_build_generates_executable() {
    let temp_dir = create_test_project("build-test");
    let project_path = temp_dir.path().join("build-test");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("build")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("All files compiled successfully"))
        .stdout(predicate::str::contains("Build completed successfully"));

    let executable_path = project_path.join("generated/target/release/jrust_app.exe");
    assert!(
        executable_path.exists(),
        "Executable should be generated at {}",
        executable_path.display()
    );
}

#[test]
fn test_build_custom_file() {
    let temp_dir = create_test_project("custom-file-test");
    let project_path = temp_dir.path().join("custom-file-test");

    let custom_file = project_path.join("src/custom.jr");
    fs::write(
        &custom_file,
        r#"export function greet(): void {
    print("Hello from custom file!");
}
"#,
    )
    .expect("Failed to write custom file");

    // Build now compiles all .jr files in the project
    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("build")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("All files compiled successfully"))
        .stdout(predicate::str::contains("Build completed successfully"));
}

#[test]
fn test_run_executes_program() {
    let temp_dir = create_test_project("run-test");
    let project_path = temp_dir.path().join("run-test");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("run")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("jRust Demo"))
        .stdout(predicate::str::contains("Program completed successfully"));
}

#[test]
fn test_run_custom_file() {
    let temp_dir = create_test_project("run-custom-test");
    let project_path = temp_dir.path().join("run-custom-test");

    let custom_file = project_path.join("src/greet.jr");
    fs::write(
        &custom_file,
        r#"export function sayHello(): void {
    print("Greetings from jRust!");
}
"#,
    )
    .expect("Failed to write custom file");

    // Run now compiles all .jr files and runs the generated executable
    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("run")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("jRust Demo"))
        .stdout(predicate::str::contains("Program completed successfully"));
}

#[test]
fn test_check_without_project_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("check")
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("jRust project"));
}

#[test]
fn test_build_without_project_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("build")
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("jRust project"));
}

#[test]
fn test_run_without_project_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("run")
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("jRust project"));
}

#[test]
fn test_init_creates_gitignore() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "gitignore-test";

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(project_name)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let gitignore_path = temp_dir.path().join(project_name).join(".gitignore");
    let content = fs::read_to_string(&gitignore_path).expect("Failed to read .gitignore");

    assert!(
        content.contains("/target/"),
        ".gitignore should ignore /target/"
    );
    assert!(
        content.contains("/generated/"),
        ".gitignore should ignore /generated/"
    );
    assert!(
        content.contains("*.exe"),
        ".gitignore should ignore *.exe"
    );
}

#[test]
fn test_full_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "workflow-test";

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(project_name)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let project_path = temp_dir.path().join(project_name);

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("check")
        .current_dir(&project_path)
        .assert()
        .success();

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("build")
        .current_dir(&project_path)
        .assert()
        .success();

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("run")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("jRust Demo"));
}

#[test]
fn test_variable_assignment_and_print() {
    let temp_dir = create_test_project("var-test");
    let project_path = temp_dir.path().join("var-test");

    let code = r#"function printGreeting(): void {
    print("Hello, jRust!");
}

printGreeting();
"#;
    fs::write(project_path.join("src/index.jr"), code).expect("Failed to write code");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("run")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, jRust!"));
}

#[test]
fn test_multiple_functions() {
    let temp_dir = create_test_project("multi-func-test");
    let project_path = temp_dir.path().join("multi-func-test");

    let code = r#"function greetOne(): void {
    print("Hello from function one!");
}

function greetTwo(): void {
    print("Hello from function two!");
}

greetOne();
greetTwo();
"#;
    fs::write(project_path.join("src/index.jr"), code).expect("Failed to write code");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("run")
        .current_dir(&project_path)
        .assert()
        .success();
}
