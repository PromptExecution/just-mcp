use std::collections::HashMap;
use std::path::{Path, PathBuf};
use snafu::prelude::*;
use dotenvy;

/// MCP-specific environment variables that may be relevant for server operation
pub const MCP_ENVIRONMENT_VARIABLES: &[&str] = &[
    "MCP_SERVER_NAME",
    "MCP_SERVER_VERSION", 
    "MCP_LOG_LEVEL",
    "MCP_CONFIG_PATH",
    "MCP_DATA_DIR",
    "MCP_TEMP_DIR",
    "MCP_MAX_MESSAGE_SIZE",
    "MCP_TIMEOUT_SECONDS",
];

#[derive(Debug, Clone)]
pub struct McpEnvironment {
    pub variables: HashMap<String, String>,
    pub sources: Vec<EnvironmentSource>,
    pub snapshot: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub enum EnvironmentSource {
    EnvFile(PathBuf),
    ProcessEnv,
    ServerConfig(String),
    Custom(HashMap<String, String>),
}

#[derive(Debug, Snafu)]
pub enum EnvironmentError {
    #[snafu(display("Failed to load .env file {}: {}", path.display(), source))]
    EnvFileLoad { path: PathBuf, source: dotenvy::Error },
    
    #[snafu(display("Missing required MCP environment variable: {}", var_name))]
    MissingMcpVariable { var_name: String },
    
    #[snafu(display("Invalid MCP environment configuration: {}", message))]
    InvalidMcpConfig { message: String },
    
    #[snafu(display("MCP environment validation failed: {}", message))]
    McpValidationFailed { message: String },
    
    #[snafu(display("Environment snapshot error: {}", message))]
    SnapshotError { message: String },
}

pub type Result<T> = std::result::Result<T, EnvironmentError>;

impl McpEnvironment {
    pub fn new() -> Self {
        McpEnvironment {
            variables: HashMap::new(),
            sources: Vec::new(),
            snapshot: None,
        }
    }
    
    pub fn with_process_env() -> Self {
        let mut env = McpEnvironment::new();
        env.load_process_env();
        env
    }
    
    pub fn load_process_env(&mut self) {
        for (key, value) in std::env::vars() {
            self.variables.insert(key, value);
        }
        self.sources.push(EnvironmentSource::ProcessEnv);
    }
    
    pub fn load_env_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        // Use dotenvy to load the .env file
        match dotenvy::from_path(path) {
            Ok(_) => {
                // Reload process environment to pick up the new variables
                self.load_process_env();
                self.sources.push(EnvironmentSource::EnvFile(path.to_path_buf()));
                Ok(())
            }
            Err(e) => Err(EnvironmentError::EnvFileLoad {
                path: path.to_path_buf(),
                source: e,
            })
        }
    }
    
    pub fn set_server_config(&mut self, config_name: String, vars: HashMap<String, String>) {
        for (key, value) in &vars {
            self.variables.insert(key.clone(), value.clone());
        }
        self.sources.push(EnvironmentSource::ServerConfig(config_name));
    }
    
    pub fn set_custom(&mut self, vars: HashMap<String, String>) {
        for (key, value) in &vars {
            self.variables.insert(key.clone(), value.clone());
        }
        self.sources.push(EnvironmentSource::Custom(vars));
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }
    
    /// Create a snapshot of the current environment state
    pub fn create_snapshot(&mut self) {
        self.snapshot = Some(self.variables.clone());
    }
    
    /// Restore environment from snapshot
    pub fn restore_from_snapshot(&mut self) -> Result<()> {
        match &self.snapshot {
            Some(snapshot) => {
                self.variables = snapshot.clone();
                Ok(())
            }
            None => Err(EnvironmentError::SnapshotError {
                message: "No snapshot available to restore from".to_string(),
            })
        }
    }
    
    /// Clear the snapshot
    pub fn clear_snapshot(&mut self) {
        self.snapshot = None;
    }
    
    /// Get environment info for MCP introspection
    pub fn get_environment_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        
        // Add source information
        info.insert("source_count".to_string(), self.sources.len().to_string());
        info.insert("variable_count".to_string(), self.variables.len().to_string());
        info.insert("has_snapshot".to_string(), self.snapshot.is_some().to_string());
        
        // Add source types
        let source_types: Vec<String> = self.sources.iter().map(|s| match s {
            EnvironmentSource::ProcessEnv => "ProcessEnv".to_string(),
            EnvironmentSource::EnvFile(path) => format!("EnvFile({})", path.display()),
            EnvironmentSource::ServerConfig(name) => format!("ServerConfig({})", name),
            EnvironmentSource::Custom(_) => "Custom".to_string(),
        }).collect();
        info.insert("sources".to_string(), source_types.join(", "));
        
        // Add MCP-specific variables if present
        for mcp_var in MCP_ENVIRONMENT_VARIABLES {
            if let Some(value) = self.variables.get(*mcp_var) {
                info.insert(format!("mcp_{}", mcp_var.to_lowercase()), value.clone());
            }
        }
        
        info
    }
    
    pub fn expand_variables(&self, text: &str) -> Result<String> {
        let mut result = text.to_string();
        
        // Handle ${VAR} and $VAR syntax
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10; // Prevent infinite loops
        
        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;
            
            // Handle ${VAR} syntax
            while let Some(start) = result.find("${") {
                if let Some(end) = result[start..].find('}') {
                    let var_name = &result[start + 2..start + end];
                    let replacement = self.variables.get(var_name)
                        .cloned()
                        .unwrap_or_else(|| {
                            // Check system environment as fallback
                            std::env::var(var_name).unwrap_or_default()
                        });
                    
                    result.replace_range(start..start + end + 1, &replacement);
                    changed = true;
                } else {
                    break;
                }
            }
            
            // Handle $VAR syntax (simple variable names)
            let mut pos = 0;
            while let Some(dollar_pos) = result[pos..].find('$') {
                let abs_pos = pos + dollar_pos;
                
                // Skip if it's ${VAR} syntax (already handled above)
                if result.chars().nth(abs_pos + 1) == Some('{') {
                    pos = abs_pos + 1;
                    continue;
                }
                
                // Extract variable name (alphanumeric + underscore)
                let var_start = abs_pos + 1;
                let var_end = result[var_start..]
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .count() + var_start;
                
                if var_end > var_start {
                    let var_name = &result[var_start..var_end];
                    let replacement = self.variables.get(var_name)
                        .cloned()
                        .unwrap_or_else(|| {
                            // Check system environment as fallback
                            std::env::var(var_name).unwrap_or_default()
                        });
                    
                    result.replace_range(abs_pos..var_end, &replacement);
                    changed = true;
                    pos = abs_pos + replacement.len();
                } else {
                    pos = abs_pos + 1;
                }
            }
        }
        
        if iterations >= MAX_ITERATIONS {
            return Err(EnvironmentError::InvalidMcpConfig {
                message: "Too many variable expansion iterations - possible circular reference".to_string(),
            });
        }
        
        Ok(result)
    }
}

impl Default for McpEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// Load MCP environment from multiple sources
pub fn load_mcp_environment(sources: &[EnvironmentSource]) -> Result<McpEnvironment> {
    let mut env = McpEnvironment::new();
    
    for source in sources {
        match source {
            EnvironmentSource::ProcessEnv => {
                env.load_process_env();
            }
            EnvironmentSource::EnvFile(path) => {
                env.load_env_file(path)?;
            }
            EnvironmentSource::Custom(vars) => {
                env.set_custom(vars.clone());
            }
            EnvironmentSource::ServerConfig(config_name) => {
                // Load server-specific configuration
                // This could be expanded to load from config files
                let mut config_vars = HashMap::new();
                config_vars.insert("MCP_SERVER_CONFIG".to_string(), config_name.clone());
                env.set_server_config(config_name.clone(), config_vars);
            }
        }
    }
    
    Ok(env)
}

/// Validate MCP environment has required variables
pub fn validate_mcp_environment(environment: &McpEnvironment, requirements: &[&str]) -> Result<()> {
    let missing_vars: Vec<&str> = requirements
        .iter()
        .filter(|&var| environment.get(var).is_none())
        .copied()
        .collect();
    
    if !missing_vars.is_empty() {
        return Err(EnvironmentError::McpValidationFailed {
            message: format!("Missing required variables: {}", missing_vars.join(", ")),
        });
    }
    
    Ok(())
}

/// Get environment info for MCP introspection
pub fn get_environment_info() -> HashMap<String, String> {
    let env = McpEnvironment::with_process_env();
    env.get_environment_info()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mcp_environment_new() {
        let env = McpEnvironment::new();
        assert!(env.variables.is_empty());
        assert!(env.sources.is_empty());
        assert!(env.snapshot.is_none());
    }
    
    #[test]
    fn test_mcp_environment_set_get() {
        let mut env = McpEnvironment::new();
        env.set("MCP_SERVER_NAME".to_string(), "just-mcp".to_string());
        
        assert_eq!(env.get("MCP_SERVER_NAME"), Some(&"just-mcp".to_string()));
        assert_eq!(env.get("NONEXISTENT"), None);
    }
    
    #[test]
    fn test_mcp_environment_snapshot() {
        let mut env = McpEnvironment::new();
        env.set("MCP_SERVER_NAME".to_string(), "just-mcp".to_string());
        
        // Create snapshot
        env.create_snapshot();
        assert!(env.snapshot.is_some());
        
        // Modify environment
        env.set("MCP_SERVER_NAME".to_string(), "modified".to_string());
        assert_eq!(env.get("MCP_SERVER_NAME"), Some(&"modified".to_string()));
        
        // Restore from snapshot
        env.restore_from_snapshot().unwrap();
        assert_eq!(env.get("MCP_SERVER_NAME"), Some(&"just-mcp".to_string()));
        
        // Clear snapshot
        env.clear_snapshot();
        assert!(env.snapshot.is_none());
        
        // Try to restore without snapshot should fail
        assert!(env.restore_from_snapshot().is_err());
    }
    
    #[test]
    fn test_mcp_environment_server_config() {
        let mut env = McpEnvironment::new();
        
        let mut config_vars = HashMap::new();
        config_vars.insert("MCP_LOG_LEVEL".to_string(), "debug".to_string());
        config_vars.insert("MCP_TIMEOUT_SECONDS".to_string(), "30".to_string());
        
        env.set_server_config("production".to_string(), config_vars);
        
        assert_eq!(env.get("MCP_LOG_LEVEL"), Some(&"debug".to_string()));
        assert_eq!(env.get("MCP_TIMEOUT_SECONDS"), Some(&"30".to_string()));
        assert_eq!(env.sources.len(), 1);
    }
    
    #[test]
    fn test_mcp_environment_info() {
        let mut env = McpEnvironment::new();
        env.set("MCP_SERVER_NAME".to_string(), "just-mcp".to_string());
        env.set("MCP_LOG_LEVEL".to_string(), "info".to_string());
        
        let mut custom_vars = HashMap::new();
        custom_vars.insert("CUSTOM_VAR".to_string(), "custom_value".to_string());
        env.set_custom(custom_vars);
        
        let info = env.get_environment_info();
        
        assert_eq!(info.get("variable_count"), Some(&"3".to_string()));
        assert_eq!(info.get("source_count"), Some(&"1".to_string()));
        assert_eq!(info.get("has_snapshot"), Some(&"false".to_string()));
        assert_eq!(info.get("mcp_mcp_server_name"), Some(&"just-mcp".to_string()));
        assert_eq!(info.get("mcp_mcp_log_level"), Some(&"info".to_string()));
    }
    
    #[test]
    fn test_mcp_variable_expansion() {
        let mut env = McpEnvironment::new();
        env.set("MCP_SERVER_NAME".to_string(), "just-mcp".to_string());
        env.set("MCP_LOG_LEVEL".to_string(), "debug".to_string());
        
        let result = env.expand_variables("Server: ${MCP_SERVER_NAME} (${MCP_LOG_LEVEL})").unwrap();
        assert_eq!(result, "Server: just-mcp (debug)");
        
        let result = env.expand_variables("$MCP_SERVER_NAME running").unwrap();
        assert_eq!(result, "just-mcp running");
    }
    
    #[test]
    fn test_validate_mcp_environment() {
        let mut env = McpEnvironment::new();
        env.set("MCP_SERVER_NAME".to_string(), "just-mcp".to_string());
        env.set("MCP_LOG_LEVEL".to_string(), "info".to_string());
        
        // Should pass with required variables present
        let result = validate_mcp_environment(&env, &["MCP_SERVER_NAME", "MCP_LOG_LEVEL"]);
        assert!(result.is_ok());
        
        // Should fail with missing required variable
        let result = validate_mcp_environment(&env, &["MCP_SERVER_NAME", "MCP_MISSING_VAR"]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MCP_MISSING_VAR"));
    }
    
    #[test]
    fn test_load_mcp_environment_multiple_sources() {
        let mut custom_vars = HashMap::new();
        custom_vars.insert("MCP_SERVER_NAME".to_string(), "just-mcp".to_string());
        custom_vars.insert("CUSTOM_VAR".to_string(), "custom_value".to_string());
        
        let sources = vec![
            EnvironmentSource::ProcessEnv,
            EnvironmentSource::Custom(custom_vars),
            EnvironmentSource::ServerConfig("production".to_string()),
        ];
        
        let env = load_mcp_environment(&sources).unwrap();
        
        assert_eq!(env.get("MCP_SERVER_NAME"), Some(&"just-mcp".to_string()));
        assert_eq!(env.get("CUSTOM_VAR"), Some(&"custom_value".to_string()));
        assert_eq!(env.get("MCP_SERVER_CONFIG"), Some(&"production".to_string()));
        assert_eq!(env.sources.len(), 3);
    }
    
    #[test]
    fn test_get_environment_info_function() {
        // This test will depend on the actual process environment
        let info = get_environment_info();
        
        assert!(info.contains_key("source_count"));
        assert!(info.contains_key("variable_count"));
        assert!(info.contains_key("has_snapshot"));
        assert!(info.contains_key("sources"));
    }
}