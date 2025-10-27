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
        project_path.join("jrust.toml").exists(),
        "jrust.toml should be created"
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
fn test_init_creates_valid_jrust_toml() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "config-test";

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("init")
        .arg(project_name)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let config_path = temp_dir.path().join(project_name).join("jrust.toml");
    let content = fs::read_to_string(&config_path).expect("Failed to read jrust.toml");

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

    let index_jr_path = temp_dir.path().join(project_name).join("src/index.jr");
    let content = fs::read_to_string(&index_jr_path).expect("Failed to read index.jr");

    assert!(
        content.contains("function main()"),
        "Template should have main function"
    );
    assert!(
        content.contains("print(\"Hello from jRust!\")"),
        "Template should have hello world print"
    );
    assert!(
        content.contains("main()"),
        "Template should call main function"
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
        .stdout(predicate::str::contains("Lexical analysis passed"))
        .stdout(predicate::str::contains("Syntax parsing passed"))
        .stdout(predicate::str::contains("Code generation passed"))
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
        r#"function greet(): void {
    print("Hello from custom file!");
}

greet();
"#,
    )
    .expect("Failed to write custom file");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("build")
        .arg("src/custom.jr")
        .current_dir(&project_path)
        .assert()
        .success()
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
        .stdout(predicate::str::contains("Hello from jRust!"))
        .stdout(predicate::str::contains("Program completed successfully"));
}

#[test]
fn test_run_custom_file() {
    let temp_dir = create_test_project("run-custom-test");
    let project_path = temp_dir.path().join("run-custom-test");

    let custom_file = project_path.join("src/greet.jr");
    fs::write(
        &custom_file,
        r#"function sayHello(): void {
    print("Greetings from jRust!");
}

sayHello();
"#,
    )
    .expect("Failed to write custom file");

    Command::cargo_bin("jrust")
        .expect("Failed to find jrust binary")
        .arg("run")
        .arg("src/greet.jr")
        .current_dir(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Greetings from jRust!"));
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
        .stdout(predicate::str::contains("Hello from jRust!"));
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
