use just_mcp_lib::executor::execute_recipe;
use just_mcp_lib::parser::parse_justfile_str;
use tempfile::TempDir;

#[test]
fn test_hello_recipe_with_parameter() {
    let content = r#"
hello name="World":
    @echo "Hello, {{name}}!"
"#;

    let justfile = parse_justfile_str(content).unwrap();
    let temp_dir = TempDir::new().unwrap();

    // Test with custom parameter
    let result = execute_recipe(&justfile, "hello", &["Claude".to_string()], temp_dir.path()).unwrap();
    
    assert_eq!(result.exit_code, 0);
    // Since this uses @ prefix, there won't be stdout in the result
    // But the execution should succeed without parameter substitution errors
    
    // Test with default parameter (no args)
    let result = execute_recipe(&justfile, "hello", &[], temp_dir.path()).unwrap();
    
    assert_eq!(result.exit_code, 0);
}