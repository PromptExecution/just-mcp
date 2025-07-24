use clap::{Arg, Command};
use just_mcp_lib::parser::parse_justfile;
use just_mcp_lib::executor::execute_recipe;
use just_mcp_lib::validator::{validate_with_help, get_signature_help, format_signature_help};
use just_mcp_lib::environment::get_environment_info;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("just-mcp")
        .version(env!("CARGO_PKG_VERSION"))
        .author("just-mcp contributors")
        .about("MCP Server for Justfile Integration")
        .arg(
            Arg::new("justfile")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to Justfile")
                .default_value("./justfile")
        )
        .subcommand(
            Command::new("introspect")
                .about("Parse and introspect Justfile")
        )
        .subcommand(
            Command::new("run")
                .about("Execute a recipe")
                .arg(
                    Arg::new("recipe")
                        .value_name("RECIPE")
                        .help("Name of the recipe to execute")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("args")
                        .value_name("ARGS")
                        .help("Arguments to pass to the recipe")
                        .num_args(0..)
                        .index(2)
                )
        )
        .subcommand(
            Command::new("help-recipe")
                .about("Show signature help for a recipe")
                .arg(
                    Arg::new("recipe")
                        .value_name("RECIPE")
                        .help("Name of the recipe to show help for")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("env-info")
                .about("Show MCP environment information")
        )
        .get_matches();

    let justfile_path = matches.get_one::<String>("justfile").unwrap();
    let path = Path::new(justfile_path);

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let justfile = match parse_justfile(path) {
                Ok(jf) => jf,
                Err(e) => {
                    eprintln!("Error parsing Justfile: {}", e);
                    std::process::exit(1);
                }
            };
            
            let recipe_name = sub_matches.get_one::<String>("recipe").unwrap();
            let args: Vec<String> = sub_matches
                .get_many::<String>("args")
                .unwrap_or_default()
                .cloned()
                .collect();
            
            // Find the recipe for validation
            let recipe = match justfile.recipes.iter().find(|r| r.name == *recipe_name) {
                Some(recipe) => recipe,
                None => {
                    eprintln!("Recipe '{}' not found", recipe_name);
                    std::process::exit(1);
                }
            };
            
            // Validate arguments before execution
            let validation = validate_with_help(recipe, &args);
            if !validation.is_valid {
                for error in &validation.errors {
                    eprintln!("Error: {}", error.message);
                }
                std::process::exit(1);
            }
            
            let working_dir = path.parent().unwrap_or_else(|| Path::new("."));
            
            match execute_recipe(&justfile, recipe_name, &args, working_dir) {
                Ok(result) => {
                    if !result.stdout.is_empty() {
                        print!("{}", result.stdout);
                    }
                    if !result.stderr.is_empty() {
                        eprint!("{}", result.stderr);
                    }
                    std::process::exit(result.exit_code);
                }
                Err(e) => {
                    eprintln!("Error executing recipe '{}': {}", recipe_name, e);
                    std::process::exit(1);
                }
            }
        }
        Some(("help-recipe", sub_matches)) => {
            let justfile = match parse_justfile(path) {
                Ok(jf) => jf,
                Err(e) => {
                    eprintln!("Error parsing Justfile: {}", e);
                    std::process::exit(1);
                }
            };
            
            let recipe_name = sub_matches.get_one::<String>("recipe").unwrap();
            
            let recipe = match justfile.recipes.iter().find(|r| r.name == *recipe_name) {
                Some(recipe) => recipe,
                None => {
                    eprintln!("Recipe '{}' not found", recipe_name);
                    std::process::exit(1);
                }
            };
            
            let help = get_signature_help(recipe);
            let formatted = format_signature_help(&help);
            println!("{}", formatted);
        }
        Some(("env-info", _)) => {
            let env_info = get_environment_info();
            
            println!("MCP Environment Information:");
            println!("==========================");
            
            // Display basic stats
            if let Some(var_count) = env_info.get("variable_count") {
                println!("Environment variables: {}", var_count);
            }
            if let Some(source_count) = env_info.get("source_count") {
                println!("Sources: {}", source_count);
            }
            if let Some(sources) = env_info.get("sources") {
                println!("Source types: {}", sources);
            }
            if let Some(has_snapshot) = env_info.get("has_snapshot") {
                println!("Has snapshot: {}", has_snapshot);
            }
            
            // Display MCP-specific variables
            println!("\nMCP Variables:");
            let mut found_mcp_vars = false;
            for (key, value) in &env_info {
                if key.starts_with("mcp_") {
                    let var_name = key.strip_prefix("mcp_").unwrap().to_uppercase();
                    println!("  {}: {}", var_name, value);
                    found_mcp_vars = true;
                }
            }
            if !found_mcp_vars {
                println!("  (No MCP-specific environment variables found)");
            }
        }
        Some(("introspect", _)) => {
            match parse_justfile(path) {
                Ok(justfile) => {
                    println!("Successfully parsed Justfile: {}", justfile_path);
                    println!("\nRecipes ({}):", justfile.recipes.len());
                    for recipe in &justfile.recipes {
                        println!("  {} ({})", recipe.name, recipe.parameters.len());
                        if let Some(ref doc) = recipe.documentation {
                            println!("    {}", doc);
                        }
                        if !recipe.dependencies.is_empty() {
                            println!("    depends on: {}", recipe.dependencies.join(", "));
                        }
                    }
                    
                    if !justfile.variables.is_empty() {
                        println!("\nVariables ({}):", justfile.variables.len());
                        for (key, value) in &justfile.variables {
                            println!("  {} = {}", key, value);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing Justfile: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("just-mcp v{} initialized", env!("CARGO_PKG_VERSION"));
            println!("Using Justfile: {}", justfile_path);
            println!("Commands:");
            println!("  just-mcp introspect         Parse and display Justfile information");
            println!("  just-mcp run <recipe>       Execute a recipe");
            println!("  just-mcp help-recipe <name> Show signature help for a recipe");
            println!("  just-mcp env-info           Show MCP environment information");
        }
    }
    
    Ok(())
}