use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use rmcp::schemars::{self, JsonSchema};

use rmcp::{
    tool, tool_router, tool_handler,
    model::{CallToolResult, ServerCapabilities, ServerInfo, Content, ProtocolVersion, Implementation, ErrorCode, ErrorData as McpError},
    handler::server::{ServerHandler, router::tool::ToolRouter, tool::Parameters},
};

use crate::{Justfile, Recipe};
use crate::parser::{parse_justfile_str, ParserError};
use crate::executor::{execute_recipe, ExecutionError};

#[derive(Debug, Snafu)]
pub enum McpServerError {
    #[snafu(display("Parse error: {}", source))]
    ParseFailed { source: ParserError },
    
    #[snafu(display("Execution error: {}", source))]
    ExecutionFailed { source: ExecutionError },
    
    #[snafu(display("IO error: {}", source))]
    IoError { source: std::io::Error },
    
    #[snafu(display("Serialization error: {}", source))]
    SerializationError { source: serde_json::Error },
    
    #[snafu(display("Justfile not found at path: {}", path))]
    JustfileNotFound { path: String },
    
    #[snafu(display("Recipe '{}' not found", recipe_name))]
    RecipeNotFound { recipe_name: String },
}

// Bridge snafu errors to MCP errors
impl From<McpServerError> for McpError {
    fn from(err: McpServerError) -> Self {
        McpError {
            code: ErrorCode(-1),
            message: err.to_string().into(),
            data: None,
        }
    }
}

// Parameter structs for tools
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListRecipesParams {
    pub justfile_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExecuteRecipeParams {
    pub recipe_name: String,
    pub args: Option<String>,
    pub justfile_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetRecipeInfoParams {
    pub recipe_name: String,
    pub justfile_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValidateJustfileParams {
    pub justfile_path: Option<String>,
}

// Response structs
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeInfo {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub documentation: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub default_value: Option<String>,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JustfileInfo {
    pub path: String,
    pub recipes: Vec<RecipeInfo>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionOutput {
    pub recipe_name: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub success: bool,
}

#[derive(Clone)]
pub struct JustMcpServer {
    working_dir: std::path::PathBuf,
    tool_router: ToolRouter<Self>,
}

impl JustMcpServer {
    pub fn new(working_dir: impl AsRef<Path>) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf(),
            tool_router: Self::tool_router(),
        }
    }

    fn load_justfile(&self, justfile_path: Option<&str>) -> Result<(Justfile, std::path::PathBuf), McpServerError> {
        let justfile_path = if let Some(path) = justfile_path {
            self.working_dir.join(path)
        } else {
            // Default justfile locations
            let candidates = ["justfile", "Justfile", ".justfile"];
            candidates
                .iter()
                .map(|name| self.working_dir.join(name))
                .find(|path| path.exists())
                .ok_or_else(|| McpServerError::JustfileNotFound {
                    path: self.working_dir.display().to_string(),
                })?
        };

        let content = std::fs::read_to_string(&justfile_path)
            .context(IoSnafu)?;
        
        let justfile = parse_justfile_str(&content)
            .context(ParseFailedSnafu)?;
        
        Ok((justfile, justfile_path))
    }

    fn recipe_to_info(recipe: &Recipe) -> RecipeInfo {
        RecipeInfo {
            name: recipe.name.clone(),
            parameters: recipe.parameters.iter().map(|p| ParameterInfo {
                name: p.name.clone(),
                default_value: p.default_value.clone(),
                required: p.default_value.is_none(),
            }).collect(),
            documentation: recipe.documentation.clone(),
            dependencies: recipe.dependencies.clone(),
        }
    }
}

#[tool_router]
impl JustMcpServer {
    #[tool(description = "List all available recipes in the justfile")]
    async fn list_recipes(&self, Parameters(params): Parameters<ListRecipesParams>) -> Result<CallToolResult, McpError> {
        let (justfile, path) = self.load_justfile(params.justfile_path.as_deref())?;
        
        let info = JustfileInfo {
            path: path.display().to_string(),
            recipes: justfile.recipes.iter().map(Self::recipe_to_info).collect(),
            variables: justfile.variables,
        };
        
        let content = serde_json::to_string_pretty(&info)
            .context(SerializationSnafu)?;
        
        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Execute a specific recipe with optional arguments")]
    async fn run_recipe(&self, Parameters(params): Parameters<ExecuteRecipeParams>) -> Result<CallToolResult, McpError> {
        let (justfile, _) = self.load_justfile(params.justfile_path.as_deref())?;
        
        // Parse arguments from JSON if provided
        let parsed_args: Vec<String> = if let Some(args_str) = params.args {
            serde_json::from_str(&args_str)
                .context(SerializationSnafu)?
        } else {
            Vec::new()
        };
        
        // Execute the recipe
        let result = execute_recipe(&justfile, &params.recipe_name, &parsed_args, &self.working_dir)
            .context(ExecutionFailedSnafu)?;
        
        let output = ExecutionOutput {
            recipe_name: params.recipe_name,
            stdout: result.stdout,
            stderr: result.stderr,
            exit_code: result.exit_code,
            duration_ms: result.duration_ms,
            success: result.exit_code == 0,
        };
        
        let content = serde_json::to_string_pretty(&output)
            .context(SerializationSnafu)?;
        
        if output.success {
            Ok(CallToolResult::success(vec![Content::text(content)]))
        } else {
            Ok(CallToolResult::error(vec![Content::text(content)]))
        }
    }

    #[tool(description = "Get detailed information about a specific recipe")]
    async fn get_recipe_info(&self, Parameters(params): Parameters<GetRecipeInfoParams>) -> Result<CallToolResult, McpError> {
        let (justfile, _) = self.load_justfile(params.justfile_path.as_deref())?;
        
        let recipe = justfile.recipes
            .iter()
            .find(|r| r.name == params.recipe_name)
            .ok_or_else(|| McpServerError::RecipeNotFound {
                recipe_name: params.recipe_name.clone(),
            })?;
        
        let info = Self::recipe_to_info(recipe);
        let content = serde_json::to_string_pretty(&info)
            .context(SerializationSnafu)?;
        
        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Validate the justfile for syntax and semantic errors")]
    async fn validate_justfile(&self, Parameters(params): Parameters<ValidateJustfileParams>) -> Result<CallToolResult, McpError> {
        let (justfile, path) = self.load_justfile(params.justfile_path.as_deref())?;
        
        // For now, just validate that it parsed correctly
        // TODO: Add more comprehensive validation using validate_arguments for each recipe
        let is_valid = true;
        let message = format!("Justfile parsed successfully with {} recipes", justfile.recipes.len());
        
        let result = serde_json::json!({
            "path": path.display().to_string(),
            "is_valid": is_valid,
            "message": message,
            "recipe_count": justfile.recipes.len(),
            "variable_count": justfile.variables.len(),
        });
        
        let content = serde_json::to_string_pretty(&result)
            .context(SerializationSnafu)?;
        
        Ok(CallToolResult::success(vec![Content::text(content)]))
    }
}

#[tool_handler]
impl ServerHandler for JustMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            server_info: Implementation::from_build_env(),
            instructions: Some("MCP server for Justfile integration. Provides tools to list, execute, inspect, and validate Justfile recipes.".into()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
        }
    }
}