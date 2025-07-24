use just_mcp_lib::executor::execute_recipe;
use just_mcp_lib::parser::parse_justfile_str;
use tempfile::TempDir;

#[test]
fn test_execute_simple_recipe() {
    let content = r#"
# Simple echo recipe
hello:
    echo "Hello, World!"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "hello", &[], temp_dir.path()).unwrap();
    
    println!("Result: {:?}", result);
    
    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Hello, World!"));
    assert!(result.stderr.is_empty());
    // Duration should be non-negative
    assert!(result.duration_ms >= 0);
}

#[test]
fn test_execute_recipe_with_parameters() {
    let content = r#"
greet name="World":
    echo "Hello, {{ name }}!"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    // Test with custom parameter
    let result = execute_recipe(&justfile, "greet", &["Rust".to_string()], temp_dir.path()).unwrap();
    assert!(result.stdout.contains("Hello, Rust!"));
    
    // Test with default parameter
    let result = execute_recipe(&justfile, "greet", &[], temp_dir.path()).unwrap();
    assert!(result.stdout.contains("Hello, World!"));
}

#[test]
fn test_execute_recipe_with_variables() {
    let content = r#"
version = "2.0.0"

show_version:
    echo "Building version {{ version }}"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "show_version", &[], temp_dir.path()).unwrap();
    
    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("Building version 2.0.0"));
}

#[test]
fn test_execute_recipe_with_dependencies() {
    let content = r#"
setup:
    echo "Setting up..."

build: setup
    echo "Building..."

test: build
    echo "Testing..."
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "test", &[], temp_dir.path()).unwrap();
    
    assert_eq!(result.exit_code, 0);
    // Should contain output from all dependencies
    assert!(result.stdout.contains("Setting up..."));
    assert!(result.stdout.contains("Building..."));
    assert!(result.stdout.contains("Testing..."));
}

#[test]
fn test_execute_recipe_with_quiet_command() {
    let content = r#"
quiet_task:
    @echo "This won't appear in output"
    echo "This will appear"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "quiet_task", &[], temp_dir.path()).unwrap();
    
    assert_eq!(result.exit_code, 0);
    assert!(!result.stdout.contains("This won't appear"));
    assert!(result.stdout.contains("This will appear"));
}

#[test]
fn test_execute_recipe_failure() {
    let content = r#"
fail:
    false
    echo "This should not run"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "fail", &[], temp_dir.path()).unwrap();
    
    assert_ne!(result.exit_code, 0);
    assert!(!result.stdout.contains("This should not run"));
}

#[test]
fn test_execute_nonexistent_recipe() {
    let content = r#"
existing:
    echo "I exist"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "nonexistent", &[], temp_dir.path());
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Recipe 'nonexistent' not found"));
}

#[test]
fn test_execute_recipe_missing_required_parameter() {
    let content = r#"
deploy env:
    echo "Deploying to {{ env }}"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "deploy", &[], temp_dir.path());
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Missing required parameter"));
}

#[test]
fn test_execute_recipe_too_many_arguments() {
    let content = r#"
simple:
    echo "No parameters expected"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "simple", &["unexpected".to_string()], temp_dir.path());
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Expected at most 0 arguments"));
}

#[test]
fn test_execute_recipe_working_directory() {
    let content = r#"
show_pwd:
    pwd
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "show_pwd", &[], temp_dir.path()).unwrap();
    
    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains(&temp_dir.path().to_string_lossy().to_string()));
}

#[test]
fn test_execute_recipe_multiple_commands() {
    let content = r#"
multi:
    echo "First command"
    echo "Second command"
    echo "Third command"
"#;
    
    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    let result = execute_recipe(&justfile, "multi", &[], temp_dir.path()).unwrap();
    
    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.contains("First command"));
    assert!(result.stdout.contains("Second command"));
    assert!(result.stdout.contains("Third command"));
}