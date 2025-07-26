use snafu::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::{Justfile, Recipe};

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
}

#[derive(Debug, Snafu)]
pub enum ExecutionError {
    #[snafu(display("Recipe '{}' not found", recipe_name))]
    RecipeNotFound { recipe_name: String },

    #[snafu(display("Invalid arguments for recipe '{}': {}", recipe_name, message))]
    InvalidArguments {
        recipe_name: String,
        message: String,
    },

    #[snafu(display(
        "Dependency '{}' failed for recipe '{}': {}",
        dependency,
        recipe_name,
        source
    ))]
    DependencyFailed {
        recipe_name: String,
        dependency: String,
        source: Box<ExecutionError>,
    },

    #[snafu(display("Execution failed for recipe '{}': {}", recipe_name, source))]
    ExecutionFailed {
        recipe_name: String,
        source: std::io::Error,
    },

    #[snafu(display("Parameter substitution failed: {}", message))]
    SubstitutionFailed { message: String },
}

pub type Result<T> = std::result::Result<T, ExecutionError>;

pub fn execute_recipe(
    justfile: &Justfile,
    recipe_name: &str,
    args: &[String],
    working_dir: &Path,
) -> Result<ExecutionResult> {
    let recipe = find_recipe(justfile, recipe_name)?;

    // Validate arguments against parameters
    let param_values = validate_arguments(recipe, args)?;

    // Execute dependencies first and collect their output
    let mut dependency_output = ExecutionResult {
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 0,
        duration_ms: 0,
    };

    for dep in &recipe.dependencies {
        let dep_result = execute_recipe(justfile, dep, &[], working_dir).map_err(|e| {
            ExecutionError::DependencyFailed {
                recipe_name: recipe_name.to_string(),
                dependency: dep.clone(),
                source: Box::new(e),
            }
        })?;

        // Accumulate dependency output
        if !dependency_output.stdout.is_empty() && !dep_result.stdout.is_empty() {
            dependency_output.stdout.push('\n');
        }
        dependency_output.stdout.push_str(&dep_result.stdout);

        if !dependency_output.stderr.is_empty() && !dep_result.stderr.is_empty() {
            dependency_output.stderr.push('\n');
        }
        dependency_output.stderr.push_str(&dep_result.stderr);

        dependency_output.duration_ms += dep_result.duration_ms;
        if dep_result.exit_code != 0 {
            dependency_output.exit_code = dep_result.exit_code;
        }
    }

    // Substitute parameters in recipe body
    let substituted_body = substitute_parameters(&recipe.body, &param_values, &justfile.variables)?;

    // Execute the recipe
    let mut recipe_result = execute_commands(&substituted_body, working_dir, recipe_name)?;

    // Combine dependency output with recipe output
    if !dependency_output.stdout.is_empty() {
        if !recipe_result.stdout.is_empty() {
            dependency_output.stdout.push('\n');
        }
        dependency_output.stdout.push_str(&recipe_result.stdout);
        recipe_result.stdout = dependency_output.stdout;
    }

    if !dependency_output.stderr.is_empty() {
        if !recipe_result.stderr.is_empty() {
            dependency_output.stderr.push('\n');
        }
        dependency_output.stderr.push_str(&recipe_result.stderr);
        recipe_result.stderr = dependency_output.stderr;
    }

    recipe_result.duration_ms += dependency_output.duration_ms;
    if dependency_output.exit_code != 0 {
        recipe_result.exit_code = dependency_output.exit_code;
    }

    Ok(recipe_result)
}

fn find_recipe<'a>(justfile: &'a Justfile, recipe_name: &str) -> Result<&'a Recipe> {
    justfile
        .recipes
        .iter()
        .find(|r| r.name == recipe_name)
        .ok_or_else(|| ExecutionError::RecipeNotFound {
            recipe_name: recipe_name.to_string(),
        })
}

fn validate_arguments(recipe: &Recipe, args: &[String]) -> Result<HashMap<String, String>> {
    let mut param_values = HashMap::new();
    let params = &recipe.parameters;

    // Check if we have too many arguments
    if args.len() > params.len() {
        return Err(ExecutionError::InvalidArguments {
            recipe_name: recipe.name.clone(),
            message: format!(
                "Expected at most {} arguments, got {}",
                params.len(),
                args.len()
            ),
        });
    }

    // Process provided arguments
    for (i, arg) in args.iter().enumerate() {
        if let Some(param) = params.get(i) {
            param_values.insert(param.name.clone(), arg.clone());
        }
    }

    // Fill in defaults for remaining parameters
    for param in params.iter().skip(args.len()) {
        if let Some(ref default_value) = param.default_value {
            param_values.insert(param.name.clone(), default_value.clone());
        } else {
            return Err(ExecutionError::InvalidArguments {
                recipe_name: recipe.name.clone(),
                message: format!("Missing required parameter: {}", param.name),
            });
        }
    }

    Ok(param_values)
}

fn substitute_parameters(
    body: &str,
    param_values: &HashMap<String, String>,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let mut result = body.to_string();

    // Substitute recipe parameters ({{ param_name }})
    for (name, value) in param_values {
        let pattern = format!("{{{{ {name} }}}}");
        result = result.replace(&pattern, value);
    }

    // Substitute global variables ({{ var_name }})
    for (name, value) in variables {
        let pattern = format!("{{{{ {name} }}}}");
        // Remove quotes from variable values for substitution
        let clean_value = value.trim_matches('"').trim_matches('\'');
        result = result.replace(&pattern, clean_value);
    }

    // Check for any remaining unsubstituted variables
    if result.contains("{{") && result.contains("}}") {
        return Err(ExecutionError::SubstitutionFailed {
            message: "Unresolved parameter or variable references found".to_string(),
        });
    }

    Ok(result)
}

fn execute_commands(body: &str, working_dir: &Path, recipe_name: &str) -> Result<ExecutionResult> {
    let start_time = Instant::now();
    let mut combined_stdout = String::new();
    let mut combined_stderr = String::new();
    let mut final_exit_code = 0;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Remove leading tabs/spaces from command
        let command_line = if let Some(stripped) = line.strip_prefix('\t') {
            stripped
        } else if let Some(stripped) = line.strip_prefix("    ") {
            stripped
        } else {
            line
        };

        // Handle special prefixes
        let (quiet, command_line) = if let Some(stripped) = command_line.strip_prefix('@') {
            (true, stripped)
        } else {
            (false, command_line)
        };

        // Execute the command
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(command_line)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd.output().with_context(|_| ExecutionFailedSnafu {
            recipe_name: recipe_name.to_string(),
        })?;

        // Collect output
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stdout.is_empty() && !quiet {
            if !combined_stdout.is_empty() {
                combined_stdout.push('\n');
            }
            combined_stdout.push_str(&stdout);
        }

        if !stderr.is_empty() {
            if !combined_stderr.is_empty() {
                combined_stderr.push('\n');
            }
            combined_stderr.push_str(&stderr);
        }

        // Update exit code (keep the last non-zero exit code, or stop on first failure)
        let exit_code = output.status.code().unwrap_or(-1);
        if exit_code != 0 {
            final_exit_code = exit_code;
            // Stop executing remaining commands on failure
            break;
        }
    }

    let duration = start_time.elapsed();

    Ok(ExecutionResult {
        stdout: combined_stdout,
        stderr: combined_stderr,
        exit_code: final_exit_code,
        duration_ms: duration.as_millis() as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parameter;
    use std::collections::HashMap;

    fn create_test_recipe(
        name: &str,
        params: Vec<Parameter>,
        body: &str,
        deps: Vec<&str>,
    ) -> Recipe {
        Recipe {
            name: name.to_string(),
            parameters: params,
            documentation: None,
            body: body.to_string(),
            dependencies: deps.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_find_recipe() {
        let recipe = create_test_recipe("build", vec![], "cargo build", vec![]);
        let justfile = Justfile {
            recipes: vec![recipe],
            variables: HashMap::new(),
        };

        assert!(find_recipe(&justfile, "build").is_ok());
        assert!(find_recipe(&justfile, "nonexistent").is_err());
    }

    #[test]
    fn test_validate_arguments_success() {
        let params = vec![
            Parameter {
                name: "env".to_string(),
                default_value: None,
            },
            Parameter {
                name: "target".to_string(),
                default_value: Some("prod".to_string()),
            },
        ];
        let recipe = create_test_recipe("deploy", params, "", vec![]);

        let args = vec!["staging".to_string()];
        let result = validate_arguments(&recipe, &args).unwrap();

        assert_eq!(result.get("env"), Some(&"staging".to_string()));
        assert_eq!(result.get("target"), Some(&"prod".to_string()));
    }

    #[test]
    fn test_validate_arguments_missing_required() {
        let params = vec![Parameter {
            name: "env".to_string(),
            default_value: None,
        }];
        let recipe = create_test_recipe("deploy", params, "", vec![]);

        let args = vec![];
        let result = validate_arguments(&recipe, &args);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing required parameter")
        );
    }

    #[test]
    fn test_substitute_parameters() {
        let mut param_values = HashMap::new();
        param_values.insert("env".to_string(), "staging".to_string());
        param_values.insert("port".to_string(), "8080".to_string());

        let mut variables = HashMap::new();
        variables.insert("version".to_string(), "\"1.0.0\"".to_string());

        let body = "echo 'Deploying {{ env }} on port {{ port }} version {{ version }}'";
        let result = substitute_parameters(body, &param_values, &variables).unwrap();

        assert_eq!(
            result,
            "echo 'Deploying staging on port 8080 version 1.0.0'"
        );
    }

    #[test]
    fn test_substitute_parameters_unresolved() {
        let param_values = HashMap::new();
        let variables = HashMap::new();

        let body = "echo 'Missing {{ unknown_var }}'";
        let result = substitute_parameters(body, &param_values, &variables);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unresolved parameter")
        );
    }
}
