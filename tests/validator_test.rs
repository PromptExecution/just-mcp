use just_mcp_lib::validator::{
    format_signature_help, get_signature_help, validate_arguments, validate_with_help,
};
use just_mcp_lib::{Parameter, Recipe};

fn create_test_recipe(name: &str, params: Vec<Parameter>, doc: Option<&str>) -> Recipe {
    Recipe {
        name: name.to_string(),
        parameters: params,
        documentation: doc.map(|s| s.to_string()),
        body: String::new(),
        dependencies: Vec::new(),
    }
}

#[test]
fn test_validate_complex_parameter_combinations() {
    let params = vec![
        Parameter {
            name: "required1".to_string(),
            default_value: None,
        },
        Parameter {
            name: "optional1".to_string(),
            default_value: Some("default1".to_string()),
        },
        Parameter {
            name: "required2".to_string(),
            default_value: None,
        },
        Parameter {
            name: "optional2".to_string(),
            default_value: Some("default2".to_string()),
        },
    ];
    let recipe = create_test_recipe(
        "complex",
        params,
        Some("Complex recipe with mixed parameters"),
    );

    // Valid: provide all required parameters
    let result = validate_arguments(&recipe, &["req1".to_string(), "req2".to_string()]);
    assert!(!result.is_valid); // Should fail because required2 is not provided in right position

    // Valid: provide required in correct order
    let result = validate_arguments(
        &recipe,
        &["req1".to_string(), "opt1".to_string(), "req2".to_string()],
    );
    assert!(result.is_valid);

    // Invalid: missing required parameters
    let result = validate_arguments(&recipe, &["req1".to_string()]);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].message.contains("required2"));
}

#[test]
fn test_validate_recipe_with_no_documentation() {
    let params = vec![Parameter {
        name: "param".to_string(),
        default_value: None,
    }];
    let recipe = create_test_recipe("undocumented", params, None);

    let help = get_signature_help(&recipe);
    assert_eq!(help.documentation, None);

    let formatted = format_signature_help(&help);
    assert!(formatted.contains("undocumented(param)"));
    assert!(!formatted.contains("\n\n")); // No extra newlines for missing doc
}

#[test]
fn test_validate_recipe_with_all_optional_parameters() {
    let params = vec![
        Parameter {
            name: "opt1".to_string(),
            default_value: Some("val1".to_string()),
        },
        Parameter {
            name: "opt2".to_string(),
            default_value: Some("val2".to_string()),
        },
        Parameter {
            name: "opt3".to_string(),
            default_value: Some("".to_string()),
        }, // Empty default
    ];
    let recipe = create_test_recipe("all_optional", params, Some("All parameters are optional"));

    // Valid: no arguments (all defaults used)
    let result = validate_arguments(&recipe, &[]);
    assert!(result.is_valid);

    // Valid: provide some arguments
    let result = validate_arguments(&recipe, &["custom1".to_string()]);
    assert!(result.is_valid);

    // Valid: provide all arguments
    let result = validate_arguments(
        &recipe,
        &[
            "custom1".to_string(),
            "custom2".to_string(),
            "custom3".to_string(),
        ],
    );
    assert!(result.is_valid);

    // Invalid: too many arguments
    let result = validate_arguments(
        &recipe,
        &[
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ],
    );
    assert!(!result.is_valid);
}

#[test]
fn test_validate_recipe_with_all_required_parameters() {
    let params = vec![
        Parameter {
            name: "req1".to_string(),
            default_value: None,
        },
        Parameter {
            name: "req2".to_string(),
            default_value: None,
        },
        Parameter {
            name: "req3".to_string(),
            default_value: None,
        },
    ];
    let recipe = create_test_recipe("all_required", params, Some("All parameters are required"));

    // Invalid: no arguments
    let result = validate_arguments(&recipe, &[]);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 3); // All three missing

    // Invalid: partial arguments
    let result = validate_arguments(&recipe, &["arg1".to_string(), "arg2".to_string()]);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1); // Only req3 missing

    // Valid: all arguments provided
    let result = validate_arguments(
        &recipe,
        &["arg1".to_string(), "arg2".to_string(), "arg3".to_string()],
    );
    assert!(result.is_valid);
}

#[test]
fn test_signature_help_formatting_edge_cases() {
    // Recipe with no parameters
    let recipe = create_test_recipe("simple", vec![], Some("Simple recipe"));
    let help = get_signature_help(&recipe);
    let formatted = format_signature_help(&help);

    assert!(formatted.contains("simple()"));
    assert!(formatted.contains("Simple recipe"));
    assert!(!formatted.contains("Parameters:")); // No parameter section

    // Recipe with parameters but no defaults
    let params = vec![
        Parameter {
            name: "param1".to_string(),
            default_value: None,
        },
        Parameter {
            name: "param2".to_string(),
            default_value: None,
        },
    ];
    let recipe = create_test_recipe("no_defaults", params, None);
    let help = get_signature_help(&recipe);
    let formatted = format_signature_help(&help);

    assert!(formatted.contains("no_defaults(param1, param2)"));
    assert!(formatted.contains("param1 (required)"));
    assert!(formatted.contains("param2 (required)"));
}

#[test]
fn test_validate_with_help_comprehensive() {
    let params = vec![
        Parameter {
            name: "env".to_string(),
            default_value: None,
        },
        Parameter {
            name: "region".to_string(),
            default_value: Some("us-east-1".to_string()),
        },
        Parameter {
            name: "dry_run".to_string(),
            default_value: Some("false".to_string()),
        },
    ];
    let recipe = create_test_recipe(
        "deploy",
        params,
        Some("Deploy application to specified environment"),
    );

    let result = validate_with_help(&recipe, &[]);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);

    let error_message = &result.errors[0].message;
    assert!(error_message.contains("Missing required parameter: env"));
    assert!(error_message.contains("Expected signature:"));
    assert!(error_message.contains("deploy(env, region=us-east-1, dry_run=false)"));
    assert!(error_message.contains("Deploy application to specified environment"));
    assert!(error_message.contains("env (required)"));
    assert!(error_message.contains("region (optional, default: us-east-1)"));
    assert!(error_message.contains("dry_run (optional, default: false)"));
}

#[test]
fn test_parameter_with_empty_default_value() {
    let params = vec![Parameter {
        name: "message".to_string(),
        default_value: Some("".to_string()),
    }];
    let recipe = create_test_recipe("echo", params, None);

    let help = get_signature_help(&recipe);
    let formatted = format_signature_help(&help);

    assert!(formatted.contains("echo(message=)"));
    assert!(formatted.contains("message (optional, default: none)")); // Empty string shown as 'none'
}

#[test]
fn test_validation_error_specificity() {
    let params = vec![
        Parameter {
            name: "first".to_string(),
            default_value: None,
        },
        Parameter {
            name: "second".to_string(),
            default_value: None,
        },
        Parameter {
            name: "third".to_string(),
            default_value: Some("default".to_string()),
        },
    ];
    let recipe = create_test_recipe("multi_param", params, None);

    // Test missing multiple required parameters
    let result = validate_arguments(&recipe, &[]);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 2);

    // Check that both missing required parameters are reported
    let error_params: Vec<&str> = result.errors.iter().map(|e| e.parameter.as_str()).collect();
    assert!(error_params.contains(&"first"));
    assert!(error_params.contains(&"second"));

    // Test missing one required parameter
    let result = validate_arguments(&recipe, &["value1".to_string()]);
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].parameter, "second");
}

#[test]
fn test_parameter_info_accuracy() {
    let params = vec![
        Parameter {
            name: "required_param".to_string(),
            default_value: None,
        },
        Parameter {
            name: "optional_param".to_string(),
            default_value: Some("default_value".to_string()),
        },
    ];
    let recipe = create_test_recipe(
        "test_recipe",
        params,
        Some("Test recipe for parameter info"),
    );

    let help = get_signature_help(&recipe);

    assert_eq!(help.parameters.len(), 2);

    // Check required parameter info
    let required_param = &help.parameters[0];
    assert_eq!(required_param.name, "required_param");
    assert!(required_param.required);
    assert_eq!(required_param.default_value, None);
    assert_eq!(required_param.description, None);

    // Check optional parameter info
    let optional_param = &help.parameters[1];
    assert_eq!(optional_param.name, "optional_param");
    assert!(!optional_param.required);
    assert_eq!(
        optional_param.default_value,
        Some("default_value".to_string())
    );
    assert_eq!(optional_param.description, None);
}
