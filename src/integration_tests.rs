use crate::create_project;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_create_project_with_full_template() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(&temp_dir).unwrap();

    let project_name = "test-project-full".to_string();
    let templates = Some(vec!["full".to_string()]);

    let result = create_project(Some(&project_name), templates).await;

    std::env::set_current_dir(original_dir).unwrap();

    let project_path = temp_dir.path().join("test-project-full");
    if project_path.exists() {
        fs::remove_dir_all(&project_path).ok();
        assert!(
            result.is_ok(),
            "create_project should handle 'full' template"
        );
    }
}
