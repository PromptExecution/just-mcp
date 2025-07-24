use clap::{Arg, Command};
use just_mcp_lib::parser::parse_justfile;
use just_mcp_lib::executor::execute_recipe;
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
            println!("  just-mcp introspect      Parse and display Justfile information");
            println!("  just-mcp run <recipe>    Execute a recipe");
        }
    }
    
    Ok(())
}
