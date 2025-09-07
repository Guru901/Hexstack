use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComponentConfig {
    pub dependencies: Vec<Dependency>,
    pub dev_dependencies: Option<Vec<Dependency>>,
    pub template_file: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dependency {
    pub name: String,
    pub features: Option<Vec<String>>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectTemplate {
    pub name: String,
    pub github_url: String,
}

pub struct ProjectSetup {
    pub name: String,
    pub selected_components: Vec<String>,
    pub selected_frontend: Option<String>,
    config: HashMap<String, ComponentConfig>,
    templates: HashMap<String, ProjectTemplate>,
}

impl ProjectSetup {
    pub async fn new(
        name: String,
        selected_components: Vec<String>,
        selected_frontend: Option<String>,
    ) -> Self {
        // Normalize component names to lowercase for case-insensitive matching
        let normalized_components: Vec<String> = selected_components
            .into_iter()
            .map(|comp| comp.to_lowercase())
            .collect();

        Self {
            name,
            selected_frontend,
            selected_components: normalized_components,
            config: Self::load_component_config(),
            templates: Self::load_templates().await,
        }
    }

    pub fn load_component_config() -> HashMap<String, ComponentConfig> {
        HashMap::from([
            (
                "ripress".to_string(),
                ComponentConfig {
                    dependencies: vec![Dependency {
                        name: "ripress".to_string(),
                        features: None,
                        version: None,
                    }],
                    dev_dependencies: None,
                    template_file: Some("ripress".to_string()),
                    description: "An HTTP Framework with best in class developer experience"
                        .to_string(),
                },
            ),
            (
                "wynd".to_string(),
                ComponentConfig {
                    dependencies: vec![Dependency {
                        name: "wynd".to_string(),
                        features: None,
                        version: None,
                    }],
                    dev_dependencies: None,
                    template_file: Some("wynd".to_string()),
                    description: "An Event Driven WebSocket library".to_string(),
                },
            ),
        ])
    }

    pub async fn load_templates() -> HashMap<String, ProjectTemplate> {
        HashMap::from([
            (
                "ripress".to_string(),
                ProjectTemplate {
                    name: "Ripress Basic".to_string(),
                    github_url: "https://github.com/Guru901/ripress-only".to_string(),
                },
            ),
            (
                "wynd".to_string(),
                ProjectTemplate {
                    name: "Wynd Basic".to_string(),
                    github_url: "https://github.com/Guru901/wynd-only".to_string(),
                },
            ),
            (
                "ripress_wynd".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd".to_string(),
                    github_url: "https://github.com/Guru901/ripress-wynd".to_string(),
                },
            ),
        ])
    }

    pub fn determine_template(&self) -> Option<&ProjectTemplate> {
        let components_set: std::collections::HashSet<&str> = self
            .selected_components
            .iter()
            .map(|s| s.as_str())
            .collect();

        // Priority order for template selection
        let template_priorities = [
            ("ripress_wynd", vec!["ripress", "wynd"]),
            ("ripress", vec!["ripress"]),
            ("wynd", vec!["wynd"]),
        ];

        for (template_key, required_components) in &template_priorities {
            if required_components
                .iter()
                .all(|comp| components_set.contains(comp))
            {
                // For multi-component templates, ensure we have ONLY those components
                if required_components.len() > 1 {
                    if components_set.len() == required_components.len() {
                        if let Some(template) = self.templates.get(*template_key) {
                            return Some(template);
                        }
                    }
                } else {
                    // For single component templates, allow additional components
                    if let Some(template) = self.templates.get(*template_key) {
                        return Some(template);
                    }
                }
            }
        }

        None
    }

    pub async fn build(self) -> Result<()> {
        let total_steps = self.calculate_total_steps();
        let pb = self.create_progress_bar(total_steps);

        // Step 3: Generate main.rs from template
        if let Some(template) = self.determine_template() {
            pb.set_message(format!("📝 Generating main.rs from {}...", template.name));

            let status = Command::new("git")
                .arg("clone")
                .arg(template.github_url.as_str())
                .arg(self.name.as_str())
                .status()
                .await
                .unwrap();

            if !status.success() {
                anyhow::bail!("Failed to clone template: {}", template.name);
            }

            Command::new("cd").arg(&self.name);
            Command::new("rm -rf .git");
            Command::new("git init");

            pb.inc(1);
        } else {
            pb.set_message("⚠️  No specific template found, keeping default main.rs");
            pb.inc(1);
        }

        pb.finish_with_message("✅ Project setup complete!");

        self.print_next_steps();
        Ok(())
    }

    pub fn calculate_total_steps(&self) -> u64 {
        1 + // cargo new
        self.selected_components.len() as u64 + // component dependencies
        1 + // template generation
        1 // common dependencies
    }

    fn create_progress_bar(&self, total_steps: u64) -> ProgressBar {
        let pb = ProgressBar::new(total_steps);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb
    }

    async fn create_cargo_project(&self) -> Result<()> {
        let output = Command::new("cargo")
            .arg("new")
            .arg(&self.name)
            .output()
            .await
            .context("Failed to execute cargo new")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("`cargo new` failed: {}", stderr.trim());
        }

        Ok(())
    }

    fn print_next_steps(&self) {
        println!("\n🎉 Project '{}' created successfully!", self.name);
        println!("\nNext steps:");
        println!("  cd {}", self.name);
        println!("  cargo run");

        if !self.selected_components.is_empty() {
            println!("\nComponents added:");
            for component in &self.selected_components {
                if let Some(config) = self.config.get(component) {
                    println!("  • {} - {}", component, config.description);
                }
            }
        }

        if let Some(template) = self.determine_template() {
            println!("\nTemplate used: {}", template.name);
        }
    }
}
