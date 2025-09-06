use super::setup::ProjectSetup;

#[tokio::test]
async fn test_component_name_normalization() {
    // Test that component names are normalized to lowercase
    let components = vec![
        "RIPRESS".to_string(),
        "Wynd".to_string(),
        "ripress".to_string(),
    ];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    assert_eq!(
        setup.selected_components,
        vec!["ripress", "wynd", "ripress"]
    );
}

#[tokio::test]
async fn test_template_selection_single_component() {
    // Test template selection for single components
    let components = vec!["ripress".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let template = setup.determine_template();
    assert!(template.is_some());
    assert_eq!(template.unwrap().name, "Ripress Basic");
}

#[tokio::test]
async fn test_template_selection_multiple_components() {
    // Test template selection for multiple components
    let components = vec!["ripress".to_string(), "wynd".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let template = setup.determine_template();
    assert!(template.is_some());
    assert_eq!(template.unwrap().name, "Ripress + Wynd");
}

#[tokio::test]
async fn test_template_selection_case_insensitive() {
    // Test that template selection works with different cases
    let components = vec!["RIPRESS".to_string(), "WyNd".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let template = setup.determine_template();
    assert!(template.is_some());
    assert_eq!(template.unwrap().name, "Ripress + Wynd");
}

#[tokio::test]
async fn test_template_selection_wynd_only() {
    // Test template selection for Wynd only
    let components = vec!["wynd".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let template = setup.determine_template();
    assert!(template.is_some());
    assert_eq!(template.unwrap().name, "Wynd Basic");
}

#[tokio::test]
async fn test_template_selection_no_match() {
    // Test template selection when no components match
    let components = vec!["unknown".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let template = setup.determine_template();
    assert!(template.is_none());
}

#[tokio::test]
async fn test_component_config_loading() {
    // Test that component configuration is loaded correctly
    let config = ProjectSetup::load_component_config();

    // Test Ripress config
    assert!(config.contains_key("ripress"));
    let ripress_config = config.get("ripress").unwrap();
    assert_eq!(ripress_config.dependencies.len(), 1);
    assert_eq!(ripress_config.dependencies[0].name, "ripress");
    assert_eq!(ripress_config.template_file, Some("ripress".to_string()));

    // Test Wynd config
    assert!(config.contains_key("wynd"));
    let wynd_config = config.get("wynd").unwrap();
    assert_eq!(wynd_config.dependencies.len(), 1);
    assert_eq!(wynd_config.dependencies[0].name, "wynd");
    assert_eq!(wynd_config.template_file, Some("wynd".to_string()));
}

#[tokio::test]
async fn test_template_loading() {
    // Test that templates are loaded correctly
    let templates = ProjectSetup::load_templates().await;

    // Test ripress template
    assert!(templates.contains_key("ripress"));
    let ripress_template = templates.get("ripress").unwrap();
    assert_eq!(ripress_template.name, "Ripress Basic");
    assert_eq!(ripress_template.dependencies, vec!["ripress"]);

    // Test wynd template
    assert!(templates.contains_key("wynd"));
    let wynd_template = templates.get("wynd").unwrap();
    assert_eq!(wynd_template.name, "Wynd Basic");
    assert_eq!(wynd_template.dependencies, vec!["wynd"]);

    // Test combined template
    assert!(templates.contains_key("ripress_wynd"));
    let combined_template = templates.get("ripress_wynd").unwrap();
    assert_eq!(combined_template.name, "Ripress + Wynd");
    assert_eq!(combined_template.dependencies, vec!["ripress", "wynd"]);
}

#[tokio::test]
async fn test_calculate_total_steps() {
    // Test step calculation for different component combinations
    let components = vec!["ripress".to_string(), "wynd".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let total_steps = setup.calculate_total_steps();
    // 1 (cargo new) + 2 (components) + 1 (template) + 1 (common deps) = 5
    assert_eq!(total_steps, 5);

    let single_component = vec!["ripress".to_string()];
    let single_setup = ProjectSetup::new("test-project".to_string(), single_component).await;
    let single_steps = single_setup.calculate_total_steps();
    // 1 (cargo new) + 1 (component) + 1 (template) + 1 (common deps) = 4
    assert_eq!(single_steps, 4);
}

#[tokio::test]
async fn test_empty_components() {
    // Test behavior with empty component list
    let components = vec![];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let template = setup.determine_template();
    assert!(template.is_none());

    let total_steps = setup.calculate_total_steps();
    // 1 (cargo new) + 0 (components) + 1 (template) + 1 (common deps) = 3
    assert_eq!(total_steps, 3);
}

#[tokio::test]
async fn test_duplicate_components() {
    // Test behavior with duplicate components
    let components = vec![
        "ripress".to_string(),
        "RIPRESS".to_string(),
        "wynd".to_string(),
    ];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    // Should normalize duplicates
    assert_eq!(setup.selected_components.len(), 3);
    assert_eq!(
        setup.selected_components,
        vec!["ripress", "ripress", "wynd"]
    );

    // Should still select the combined template
    let template = setup.determine_template();
    assert!(template.is_some());
    assert_eq!(template.unwrap().name, "Ripress + Wynd");
}

#[tokio::test]
async fn test_component_set_creation() {
    // Test that component sets are created correctly for matching
    let components = vec!["ripress".to_string(), "wynd".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let components_set: std::collections::HashSet<&str> = setup
        .selected_components
        .iter()
        .map(|s| s.as_str())
        .collect();

    assert!(components_set.contains("ripress"));
    assert!(components_set.contains("wynd"));
    assert_eq!(components_set.len(), 2);
}

#[tokio::test]
async fn test_template_priority_order() {
    // Test that template priority order works correctly
    let components = vec!["ripress".to_string(), "wynd".to_string()];
    let setup = ProjectSetup::new("test-project".to_string(), components).await;

    let template = setup.determine_template();
    assert!(template.is_some());

    // Should select the combined template (highest priority)
    assert_eq!(template.unwrap().name, "Ripress + Wynd");

    // Test with only ripress - should select ripress template
    let ripress_only = vec!["ripress".to_string()];
    let ripress_setup = ProjectSetup::new("test-project".to_string(), ripress_only).await;
    let ripress_template = ripress_setup.determine_template();
    assert!(ripress_template.is_some());
    assert_eq!(ripress_template.unwrap().name, "Ripress Basic");
}

#[tokio::test]
async fn test_case_insensitive_component_matching() {
    // Test various case combinations
    let test_cases = vec![
        vec!["RIPRESS".to_string(), "WYND".to_string()],
        vec!["Ripress".to_string(), "Wynd".to_string()],
        vec!["ripress".to_string(), "wynd".to_string()],
        vec!["RiPrEsS".to_string(), "WyNd".to_string()],
    ];

    for components in test_cases {
        let setup = ProjectSetup::new("test-project".to_string(), components).await;
        let template = setup.determine_template();
        assert!(
            template.is_some(),
            "Failed for components: {:?}",
            setup.selected_components
        );
        assert_eq!(template.unwrap().name, "Ripress + Wynd");
    }
}
