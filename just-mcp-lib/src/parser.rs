use snafu::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::{Justfile, Parameter, Recipe};

#[derive(Debug, Snafu)]
pub enum ParserError {
    #[snafu(display("Failed to read file {}: {}", path.display(), source))]
    FileRead {
        path: std::path::PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Parse error at line {}: {}", line, message))]
    ParseError { line: usize, message: String },

    #[snafu(display("Invalid recipe syntax: {}", message))]
    InvalidRecipe { message: String },
}

pub type Result<T> = std::result::Result<T, ParserError>;

pub fn parse_justfile(path: &Path) -> Result<Justfile> {
    let content = fs::read_to_string(path).context(FileReadSnafu { path })?;
    parse_justfile_str(&content)
}

pub fn parse_justfile_str(content: &str) -> Result<Justfile> {
    let mut recipes = Vec::new();
    let mut variables = HashMap::new();
    let mut current_recipe: Option<Recipe> = None;
    let mut current_doc: Option<String> = None;
    for (line_number, line) in content.lines().enumerate() {
        let line_number = line_number + 1;
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Handle comments and documentation
        if let Some(stripped) = trimmed.strip_prefix('#') {
            let comment = stripped.trim();
            if !comment.is_empty() {
                current_doc = Some(comment.to_string());
            }
            continue;
        }

        // Handle variable assignments
        if let Some((key, value)) = parse_variable_assignment(trimmed) {
            variables.insert(key, value);
            continue;
        }

        // Handle recipe definitions
        if let Some(recipe) = parse_recipe_line(trimmed, current_doc.take())? {
            // If we have a current recipe, save it
            if let Some(existing_recipe) = current_recipe.take() {
                recipes.push(existing_recipe);
            }

            current_recipe = Some(recipe);
            continue;
        }

        // Handle recipe body lines (indented)
        if line.starts_with('\t') || line.starts_with("    ") {
            if let Some(ref mut recipe) = current_recipe {
                if !recipe.body.is_empty() {
                    recipe.body.push('\n');
                }
                recipe.body.push_str(line);
            }
            continue;
        }

        // If we reach here with a non-empty line that doesn't match patterns, it's an error
        if !trimmed.is_empty() {
            return Err(ParserError::ParseError {
                line: line_number,
                message: format!("Unexpected content: {trimmed}"),
            });
        }
    }

    // Don't forget the last recipe
    if let Some(recipe) = current_recipe {
        recipes.push(recipe);
    }

    Ok(Justfile { recipes, variables })
}

fn parse_variable_assignment(line: &str) -> Option<(String, String)> {
    if let Some((key, value)) = line.split_once('=') {
        let key = key.trim();
        let value = value.trim();

        // Basic validation - key must be a valid identifier
        if key.chars().all(|c| c.is_alphanumeric() || c == '_') && !key.is_empty() {
            return Some((key.to_string(), value.to_string()));
        }
    }
    None
}

fn parse_recipe_line(line: &str, documentation: Option<String>) -> Result<Option<Recipe>> {
    // Recipe format: name param1 param2='default' *param3: dependency1 dependency2
    if let Some(colon_pos) = line.find(':') {
        let (header, deps_part) = line.split_at(colon_pos);
        let deps_part = deps_part[1..].trim(); // Remove the ':'

        let header = header.trim();
        let parts: Vec<&str> = header.split_whitespace().collect();

        if parts.is_empty() {
            return Ok(None);
        }

        let name = parts[0].to_string();
        let mut parameters = Vec::new();

        // Parse parameters
        for param_str in &parts[1..] {
            let parameter = parse_parameter(param_str)?;
            parameters.push(parameter);
        }

        // Parse dependencies
        let dependencies: Vec<String> = if deps_part.is_empty() {
            Vec::new()
        } else {
            deps_part
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        };

        return Ok(Some(Recipe {
            name,
            parameters,
            documentation,
            body: String::new(),
            dependencies,
        }));
    }

    Ok(None)
}

fn parse_parameter(param_str: &str) -> Result<Parameter> {
    if let Some((name, default)) = param_str.split_once('=') {
        // Parameter with default value
        let name = name.trim();
        let default = default.trim().trim_matches('"').trim_matches('\'');

        Ok(Parameter {
            name: name.to_string(),
            default_value: Some(default.to_string()),
        })
    } else {
        // Parameter without default
        let name = param_str.trim();

        // Handle variadic parameters (prefixed with *)
        let name = if let Some(stripped) = name.strip_prefix('*') {
            stripped
        } else {
            name
        };

        Ok(Parameter {
            name: name.to_string(),
            default_value: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_recipe() {
        let content = r#"
# Build the project
build:
    cargo build
"#;

        let justfile = parse_justfile_str(content).unwrap();
        assert_eq!(justfile.recipes.len(), 1);

        let recipe = &justfile.recipes[0];
        assert_eq!(recipe.name, "build");
        assert_eq!(recipe.documentation, Some("Build the project".to_string()));
        assert!(recipe.parameters.is_empty());
        assert!(recipe.dependencies.is_empty());
        assert!(recipe.body.contains("cargo build"));
    }

    #[test]
    fn test_parse_recipe_with_parameters() {
        let content = r#"
deploy env target='production':
    echo "Deploying to {{ env }} {{ target }}"
"#;

        let justfile = parse_justfile_str(content).unwrap();
        assert_eq!(justfile.recipes.len(), 1);

        let recipe = &justfile.recipes[0];
        assert_eq!(recipe.name, "deploy");
        assert_eq!(recipe.parameters.len(), 2);
        assert_eq!(recipe.parameters[0].name, "env");
        assert_eq!(recipe.parameters[0].default_value, None);
        assert_eq!(recipe.parameters[1].name, "target");
        assert_eq!(
            recipe.parameters[1].default_value,
            Some("production".to_string())
        );
    }

    #[test]
    fn test_parse_recipe_with_dependencies() {
        let content = r#"
test: build
    cargo test

build:
    cargo build
"#;

        let justfile = parse_justfile_str(content).unwrap();
        assert_eq!(justfile.recipes.len(), 2);

        let test_recipe = &justfile.recipes[0];
        assert_eq!(test_recipe.name, "test");
        assert_eq!(test_recipe.dependencies, vec!["build"]);
    }

    #[test]
    fn test_parse_variables() {
        let content = r#"
version = "1.0.0"
debug = true

build:
    echo "Building version {{ version }}"
"#;

        let justfile = parse_justfile_str(content).unwrap();
        assert_eq!(justfile.variables.len(), 2);
        assert_eq!(
            justfile.variables.get("version"),
            Some(&"\"1.0.0\"".to_string())
        );
        assert_eq!(justfile.variables.get("debug"), Some(&"true".to_string()));
    }
}
