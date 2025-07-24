use just_mcp_lib::parser::parse_justfile;
use std::path::Path;

#[test]
fn test_parse_sample_justfile() {
    let justfile_path = Path::new("test-fixtures/sample.justfile");
    let justfile = parse_justfile(justfile_path).expect("Failed to parse sample Justfile");

    // Check variables
    assert_eq!(justfile.variables.len(), 2);
    assert_eq!(justfile.variables.get("version"), Some(&"\"1.0.0\"".to_string()));
    assert_eq!(justfile.variables.get("debug"), Some(&"false".to_string()));

    // Check recipes
    assert!(!justfile.recipes.is_empty());

    // Find specific recipes
    let build_recipe = justfile.recipes.iter().find(|r| r.name == "build").unwrap();
    assert_eq!(build_recipe.documentation, Some("Build the project".to_string()));
    assert!(build_recipe.body.contains("cargo build --release"));
    assert!(build_recipe.dependencies.is_empty());

    let test_recipe = justfile.recipes.iter().find(|r| r.name == "test").unwrap();
    assert_eq!(test_recipe.dependencies, vec!["build"]);

    let deploy_recipe = justfile.recipes.iter().find(|r| r.name == "deploy").unwrap();
    assert_eq!(deploy_recipe.parameters.len(), 2);
    assert_eq!(deploy_recipe.parameters[0].name, "env");
    assert_eq!(deploy_recipe.parameters[0].default_value, None);
    assert_eq!(deploy_recipe.parameters[1].name, "target");
    assert_eq!(deploy_recipe.parameters[1].default_value, Some("production".to_string()));
    assert_eq!(deploy_recipe.dependencies, vec!["build", "test"]);

    let serve_recipe = justfile.recipes.iter().find(|r| r.name == "serve").unwrap();
    assert_eq!(serve_recipe.parameters.len(), 1);
    assert_eq!(serve_recipe.parameters[0].name, "port");
    assert_eq!(serve_recipe.parameters[0].default_value, Some("8080".to_string()));
}