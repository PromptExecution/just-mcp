pub mod parser;
pub mod executor;
pub mod validator;
pub mod environment;
pub mod mcp_server;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Justfile {
    pub recipes: Vec<Recipe>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub documentation: Option<String>,
    pub body: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<String>,
}
