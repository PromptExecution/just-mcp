use crate::Recipe;
use snafu::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub parameter: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignatureHelp {
    pub recipe_name: String,
    pub parameters: Vec<ParameterInfo>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterInfo {
    pub name: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Snafu)]
pub enum ValidationSnafu {
    #[snafu(display("Recipe '{}' not found", recipe_name))]
    RecipeNotFound { recipe_name: String },
}

pub type Result<T> = std::result::Result<T, ValidationSnafu>;

/// Validate arguments against recipe parameters
pub fn validate_arguments(recipe: &Recipe, args: &[String]) -> ValidationResult {
    let mut errors = Vec::new();
    let params = &recipe.parameters;

    // Check if we have too many arguments
    if args.len() > params.len() {
        errors.push(ValidationError {
            parameter: "<extra>".to_string(),
            message: format!(
                "Too many arguments: expected at most {}, got {}",
                params.len(),
                args.len()
            ),
        });
    }

    // Check each parameter
    for (i, param) in params.iter().enumerate() {
        if i >= args.len() {
            // No argument provided for this parameter
            if param.default_value.is_none() {
                errors.push(ValidationError {
                    parameter: param.name.clone(),
                    message: format!("Missing required parameter: {}", param.name),
                });
            }
        }
        // If an argument is provided, it's valid (we don't do type checking yet)
    }

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
    }
}

/// Get signature help for a recipe
pub fn get_signature_help(recipe: &Recipe) -> SignatureHelp {
    let parameters = recipe
        .parameters
        .iter()
        .map(|param| ParameterInfo {
            name: param.name.clone(),
            required: param.default_value.is_none(),
            default_value: param.default_value.clone(),
            description: None, // Could be enhanced to parse parameter documentation
        })
        .collect();

    SignatureHelp {
        recipe_name: recipe.name.clone(),
        parameters,
        documentation: recipe.documentation.clone(),
    }
}

/// Format signature help for display
pub fn format_signature_help(help: &SignatureHelp) -> String {
    let mut result = String::new();

    // Recipe name and parameters
    result.push_str(&format!("{}(", help.recipe_name));

    let param_strings: Vec<String> = help
        .parameters
        .iter()
        .map(|param| {
            if param.required {
                param.name.clone()
            } else {
                format!(
                    "{}={}",
                    param.name,
                    param.default_value.as_deref().unwrap_or("")
                )
            }
        })
        .collect();

    result.push_str(&param_strings.join(", "));
    result.push(')');

    // Add documentation and parameter sections with proper spacing
    let has_doc = help.documentation.is_some();
    let has_params = !help.parameters.is_empty();

    // Documentation
    if let Some(ref doc) = help.documentation {
        result.push_str(&format!("\n\n{}", doc));
    }

    // Parameter details
    if has_params {
        if has_doc {
            result.push_str("\n\nParameters:");
        } else {
            result.push_str("\nParameters:");
        }
        for param in &help.parameters {
            result.push_str(&format!("\n  {}", param.name));
            if param.required {
                result.push_str(" (required)");
            } else {
                let default_display = match param.default_value.as_deref() {
                    Some("") => "none",
                    Some(val) => val,
                    None => "none",
                };
                result.push_str(&format!(" (optional, default: {})", default_display));
            }
            if let Some(ref desc) = param.description {
                result.push_str(&format!(" - {}", desc));
            }
        }
    }

    result
}

/// Validate arguments and provide helpful error messages
pub fn validate_with_help(recipe: &Recipe, args: &[String]) -> ValidationResult {
    let mut result = validate_arguments(recipe, args);

    // Enhance error messages with signature help
    if !result.is_valid {
        let help = get_signature_help(recipe);
        let formatted_help = format_signature_help(&help);

        // Add signature help to the first error
        if let Some(first_error) = result.errors.first_mut() {
            first_error.message = format!(
                "{}\n\nExpected signature:\n{}",
                first_error.message, formatted_help
            );
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parameter;

    fn create_test_recipe(name: &str, params: Vec<Parameter>) -> Recipe {
        Recipe {
            name: name.to_string(),
            parameters: params,
            documentation: Some(format!("Test recipe {}", name)),
            body: String::new(),
            dependencies: Vec::new(),
        }
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
        let recipe = create_test_recipe("deploy", params);

        // Valid: provide required parameter, use default for optional
        let result = validate_arguments(&recipe, &["staging".to_string()]);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());

        // Valid: provide both parameters
        let result = validate_arguments(&recipe, &["staging".to_string(), "dev".to_string()]);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_arguments_missing_required() {
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
        let recipe = create_test_recipe("deploy", params);

        let result = validate_arguments(&recipe, &[]);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].parameter, "env");
        assert!(
            result.errors[0]
                .message
                .contains("Missing required parameter")
        );
    }

    #[test]
    fn test_validate_arguments_too_many() {
        let params = vec![Parameter {
            name: "env".to_string(),
            default_value: None,
        }];
        let recipe = create_test_recipe("deploy", params);

        let result = validate_arguments(&recipe, &["staging".to_string(), "extra".to_string()]);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].parameter, "<extra>");
        assert!(result.errors[0].message.contains("Too many arguments"));
    }

    #[test]
    fn test_validate_arguments_no_parameters() {
        let recipe = create_test_recipe("build", vec![]);

        // Valid: no args for no params
        let result = validate_arguments(&recipe, &[]);
        assert!(result.is_valid);

        // Invalid: args for no params
        let result = validate_arguments(&recipe, &["unexpected".to_string()]);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_get_signature_help() {
        let params = vec![
            Parameter {
                name: "env".to_string(),
                default_value: None,
            },
            Parameter {
                name: "target".to_string(),
                default_value: Some("prod".to_string()),
            },
            Parameter {
                name: "verbose".to_string(),
                default_value: Some("false".to_string()),
            },
        ];
        let recipe = create_test_recipe("deploy", params);

        let help = get_signature_help(&recipe);

        assert_eq!(help.recipe_name, "deploy");
        assert_eq!(help.parameters.len(), 3);
        assert_eq!(help.documentation, Some("Test recipe deploy".to_string()));

        // Check parameter info
        assert_eq!(help.parameters[0].name, "env");
        assert!(help.parameters[0].required);
        assert_eq!(help.parameters[0].default_value, None);

        assert_eq!(help.parameters[1].name, "target");
        assert!(!help.parameters[1].required);
        assert_eq!(help.parameters[1].default_value, Some("prod".to_string()));

        assert_eq!(help.parameters[2].name, "verbose");
        assert!(!help.parameters[2].required);
        assert_eq!(help.parameters[2].default_value, Some("false".to_string()));
    }

    #[test]
    fn test_format_signature_help() {
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
        let recipe = create_test_recipe("deploy", params);
        let help = get_signature_help(&recipe);

        let formatted = format_signature_help(&help);

        assert!(formatted.contains("deploy(env, target=prod)"));
        assert!(formatted.contains("Test recipe deploy"));
        assert!(formatted.contains("env (required)"));
        assert!(formatted.contains("target (optional, default: prod)"));
    }

    #[test]
    fn test_validate_with_help() {
        let params = vec![Parameter {
            name: "env".to_string(),
            default_value: None,
        }];
        let recipe = create_test_recipe("deploy", params);

        let result = validate_with_help(&recipe, &[]);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert!(
            result.errors[0]
                .message
                .contains("Missing required parameter")
        );
        assert!(result.errors[0].message.contains("Expected signature"));
        assert!(result.errors[0].message.contains("deploy(env)"));
    }
}
