use just_mcp_lib::environment::{
    EnvironmentSource, MCP_ENVIRONMENT_VARIABLES, McpEnvironment, get_environment_info,
    load_mcp_environment, validate_mcp_environment,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_mcp_environment_integration() {
    // Test creating an MCP environment with multiple sources
    let temp_dir = TempDir::new().unwrap();
    let env_file = temp_dir.path().join(".env");

    // Create a .env file with MCP variables
    let mut file = File::create(&env_file).unwrap();
    writeln!(file, "MCP_SERVER_NAME=just-mcp-server").unwrap();
    writeln!(file, "MCP_LOG_LEVEL=debug").unwrap();
    writeln!(file, "MCP_TIMEOUT_SECONDS=30").unwrap();

    // Create custom configuration
    let mut server_config = HashMap::new();
    server_config.insert("MCP_MAX_MESSAGE_SIZE".to_string(), "1048576".to_string());
    server_config.insert("MCP_DATA_DIR".to_string(), "/tmp/mcp-data".to_string());

    // Load environment from multiple sources
    let sources = vec![
        EnvironmentSource::ProcessEnv,
        EnvironmentSource::EnvFile(env_file),
        EnvironmentSource::ServerConfig("production".to_string()),
        EnvironmentSource::Custom(server_config),
    ];

    let env = load_mcp_environment(&sources).unwrap();

    // Verify MCP-specific variables are loaded
    assert!(env.get("MCP_SERVER_NAME").is_some());
    assert!(env.get("MCP_LOG_LEVEL").is_some());
    assert!(env.get("MCP_TIMEOUT_SECONDS").is_some());
    assert!(env.get("MCP_MAX_MESSAGE_SIZE").is_some());
    assert!(env.get("MCP_DATA_DIR").is_some());
    assert!(env.get("MCP_SERVER_CONFIG").is_some());

    // Test validation
    let required_vars = ["MCP_SERVER_NAME", "MCP_LOG_LEVEL"];
    let validation = validate_mcp_environment(&env, &required_vars);
    assert!(validation.is_ok());

    // Test environment info
    let info = env.get_environment_info();
    assert!(info.contains_key("source_count"));
    assert!(info.contains_key("variable_count"));
    assert!(info.get("source_count").unwrap().parse::<usize>().unwrap() > 0);
}

#[test]
fn test_mcp_environment_snapshot_workflow() {
    let mut env = McpEnvironment::new();

    // Set initial MCP configuration
    env.set("MCP_SERVER_NAME".to_string(), "test-server".to_string());
    env.set("MCP_LOG_LEVEL".to_string(), "info".to_string());

    // Create snapshot for testing
    env.create_snapshot();

    // Modify environment for testing
    env.set("MCP_LOG_LEVEL".to_string(), "debug".to_string());
    env.set("MCP_TIMEOUT_SECONDS".to_string(), "60".to_string());

    assert_eq!(env.get("MCP_LOG_LEVEL"), Some(&"debug".to_string()));
    assert_eq!(env.get("MCP_TIMEOUT_SECONDS"), Some(&"60".to_string()));

    // Restore original environment
    env.restore_from_snapshot().unwrap();

    assert_eq!(env.get("MCP_LOG_LEVEL"), Some(&"info".to_string()));
    assert_eq!(env.get("MCP_TIMEOUT_SECONDS"), None);

    // Test variable expansion with MCP variables
    let expanded = env
        .expand_variables("Server: ${MCP_SERVER_NAME} at ${MCP_LOG_LEVEL} level")
        .unwrap();
    assert_eq!(expanded, "Server: test-server at info level");
}

#[test]
fn test_mcp_environment_validation_failures() {
    let mut env = McpEnvironment::new();
    env.set("MCP_SERVER_NAME".to_string(), "test-server".to_string());

    // Test missing required variables
    let required_vars = ["MCP_SERVER_NAME", "MCP_LOG_LEVEL", "MCP_MISSING_VAR"];
    let validation = validate_mcp_environment(&env, &required_vars);

    assert!(validation.is_err());
    let error_msg = validation.unwrap_err().to_string();
    assert!(error_msg.contains("MCP_LOG_LEVEL"));
    assert!(error_msg.contains("MCP_MISSING_VAR"));
    assert!(!error_msg.contains("MCP_SERVER_NAME")); // This one is present
}

#[test]
fn test_get_environment_info_global_function() {
    // Test the global environment info function
    let info = get_environment_info();

    // Should contain basic info keys
    assert!(info.contains_key("source_count"));
    assert!(info.contains_key("variable_count"));
    assert!(info.contains_key("has_snapshot"));
    assert!(info.contains_key("sources"));

    // Should be able to parse counts
    let var_count: usize = info.get("variable_count").unwrap().parse().unwrap();
    assert!(var_count > 0); // Should have at least some environment variables
}

#[test]
fn test_mcp_environment_constants() {
    // Test that our MCP environment constants are comprehensive
    assert!(MCP_ENVIRONMENT_VARIABLES.contains(&"MCP_SERVER_NAME"));
    assert!(MCP_ENVIRONMENT_VARIABLES.contains(&"MCP_LOG_LEVEL"));
    assert!(MCP_ENVIRONMENT_VARIABLES.contains(&"MCP_TIMEOUT_SECONDS"));
    assert!(MCP_ENVIRONMENT_VARIABLES.contains(&"MCP_MAX_MESSAGE_SIZE"));
    assert!(MCP_ENVIRONMENT_VARIABLES.len() >= 5);
}

#[test]
fn test_mcp_environment_variable_expansion_edge_cases() {
    let mut env = McpEnvironment::new();
    env.set("MCP_PREFIX".to_string(), "test".to_string());
    env.set("MCP_SUFFIX".to_string(), "server".to_string());

    // Test mixed syntax
    let result = env.expand_variables("${MCP_PREFIX}_$MCP_SUFFIX").unwrap();
    assert_eq!(result, "test_server");

    // Test missing variables (should expand to empty)
    let result = env.expand_variables("${MCP_MISSING}_value").unwrap();
    assert_eq!(result, "_value");

    // Test complex expansion
    let result = env
        .expand_variables("Config: ${MCP_PREFIX}-${MCP_SUFFIX}-config")
        .unwrap();
    assert_eq!(result, "Config: test-server-config");
}

#[test]
fn test_server_config_source() {
    let mut env = McpEnvironment::new();

    let mut config_1 = HashMap::new();
    config_1.insert("MCP_CONFIG_1".to_string(), "value1".to_string());

    let mut config_2 = HashMap::new();
    config_2.insert("MCP_CONFIG_2".to_string(), "value2".to_string());

    env.set_server_config("config1".to_string(), config_1);
    env.set_server_config("config2".to_string(), config_2);

    // Should have variables from both configs
    assert_eq!(env.get("MCP_CONFIG_1"), Some(&"value1".to_string()));
    assert_eq!(env.get("MCP_CONFIG_2"), Some(&"value2".to_string()));

    // Should have source entries for both configs
    assert_eq!(env.sources.len(), 2);

    let info = env.get_environment_info();
    assert!(info.get("sources").unwrap().contains("ServerConfig"));
}
