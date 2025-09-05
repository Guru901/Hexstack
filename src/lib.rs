use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{fs::read_to_string, process::Command};

pub mod templates;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ComponentConfig {
    dependencies: Vec<Dependency>,
    dev_dependencies: Option<Vec<Dependency>>,
    template_file: Option<String>,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Dependency {
    name: String,
    features: Option<Vec<String>>,
    version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ProjectTemplate {
    name: String,
    content: String,
    dependencies: Vec<String>, // Required components
}

pub struct ProjectSetup {
    name: String,
    selected_components: Vec<String>,
    config: HashMap<String, ComponentConfig>,
    templates: HashMap<String, ProjectTemplate>,
}

impl ProjectSetup {
    pub async fn new(name: String, selected_components: Vec<String>) -> Self {
        Self {
            name,
            selected_components,
            config: Self::load_component_config(),
            templates: Self::load_templates().await,
        }
    }

    fn load_component_config() -> HashMap<String, ComponentConfig> {
        HashMap::from([
            (
                "Ripress".to_string(),
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
                "Wynd".to_string(),
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

    async fn load_templates() -> HashMap<String, ProjectTemplate> {
        HashMap::from([
            (
                "ripress".to_string(),
                ProjectTemplate {
                    name: "Ripress Basic".to_string(),
                    dependencies: vec!["Ripress".to_string()],
                    content: include_str!("./templates/ripress_basic.rs").to_string(),
                },
            ),
            (
                "wynd".to_string(),
                ProjectTemplate {
                    name: "Wynd Basic".to_string(),
                    dependencies: vec!["Wynd".to_string()],
                    content: include_str!("./templates/wynd_basic.rs").to_string(),
                },
            ),
            (
                "ripress_wynd".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd".to_string(),
                    dependencies: vec!["Ripress".to_string(), "Wynd".to_string()],
                    content: include_str!("./templates/ripress_wynd.rs").to_string(),
                },
            ),
        ])
    }

    fn determine_template(&self) -> Option<&ProjectTemplate> {
        let components_set: std::collections::HashSet<&str> = self
            .selected_components
            .iter()
            .map(|s| s.as_str())
            .collect();

        // Priority order for template selection
        let template_priorities = [
            ("ripress_wynd", vec!["Ripress", "Wynd"]),
            ("ripress", vec!["Ripress"]),
            ("wynd", vec!["Wynd"]),
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

        // Step 1: Create cargo project
        pb.set_message("📦 Creating cargo project...");
        self.create_cargo_project()
            .await
            .context("Failed to create cargo project")?;
        pb.inc(1);

        // Step 2: Add dependencies
        for component in &self.selected_components {
            pb.set_message(format!("📥 Adding {}...", component));
            self.add_component_dependencies(component)
                .await
                .with_context(|| format!("Failed to add dependencies for {}", component))?;
            pb.inc(1);
        }

        // Step 3: Generate main.rs from template
        if let Some(template) = self.determine_template() {
            pb.set_message(format!("📝 Generating main.rs from {}...", template.name));
            self.write_main_file(template)
                .await
                .context("Failed to generate main.rs from template")?;
            pb.inc(1);
        } else {
            pb.set_message("⚠️  No specific template found, keeping default main.rs");
            pb.inc(1);
        }

        // Step 4: Add common dependencies if needed
        pb.set_message("🔧 Adding common dependencies...");
        self.add_common_dependencies()
            .await
            .context("Failed to add common dependencies")?;
        pb.inc(1);

        pb.finish_with_message("✅ Project setup complete!");

        self.print_next_steps();
        Ok(())
    }

    fn calculate_total_steps(&self) -> u64 {
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
        let status = Command::new("cargo")
            .arg("new")
            .arg(&self.name)
            .status()
            .await
            .context("Failed to execute cargo new")?;

        if !status.success() {
            anyhow::bail!("Cargo new command failed");
        }

        Ok(())
    }

    async fn add_component_dependencies(&self, component: &str) -> Result<()> {
        if let Some(config) = self.config.get(component) {
            // Add regular dependencies
            for dep in &config.dependencies {
                self.add_dependency(
                    &dep.name,
                    dep.features.as_deref(),
                    dep.version.as_deref(),
                    false,
                )
                .await?;
            }

            // Add dev dependencies if any
            if let Some(dev_deps) = &config.dev_dependencies {
                for dep in dev_deps {
                    self.add_dependency(
                        &dep.name,
                        dep.features.as_deref(),
                        dep.version.as_deref(),
                        true,
                    )
                    .await?;
                }
            }
        }
        Ok(())
    }

    async fn add_dependency(
        &self,
        name: &str,
        features: Option<&[String]>,
        version: Option<&str>,
        is_dev: bool,
    ) -> Result<()> {
        let mut cmd = Command::new("cargo");
        cmd.arg("add").arg(name).current_dir(&self.name);

        if is_dev {
            cmd.arg("--dev");
        }

        if let Some(feat) = features {
            if !feat.is_empty() {
                cmd.arg("--features").arg(feat.join(","));
            }
        }

        if let Some(ver) = version {
            cmd.arg("--version").arg(ver);
        }

        let status = cmd
            .status()
            .await
            .with_context(|| format!("Failed to execute cargo add for {}", name))?;

        if !status.success() {
            anyhow::bail!("Failed to add dependency: {}", name);
        }

        Ok(())
    }

    async fn write_main_file(&self, template: &ProjectTemplate) -> Result<()> {
        let main_path = format!("{}/src/main.rs", self.name);

        // Write template content directly to main.rs
        tokio::fs::write(&main_path, &template.content)
            .await
            .with_context(|| format!("Failed to write main.rs with template: {}", template.name))?;

        Ok(())
    }

    async fn add_common_dependencies(&self) -> Result<()> {
        let components_set: std::collections::HashSet<&str> = self
            .selected_components
            .iter()
            .map(|s| s.as_str())
            .collect();

        // Add tokio if we have components that need async runtime
        if components_set.contains("Ripress") || components_set.contains("Wynd") {
            self.add_dependency(
                "tokio",
                Some(&vec!["macros".to_string(), "rt-multi-thread".to_string()]),
                None,
                false,
            )
            .await?;
        }

        // Add component-specific features
        if components_set.contains("Ripress") && components_set.contains("Wynd") {
            // Update ripress with wynd feature
            self.add_dependency("ripress", Some(&vec!["with-wynd".to_string()]), None, false)
                .await?;
            // Update wynd with ripress feature
            self.add_dependency("wynd", Some(&vec!["with-ripress".to_string()]), None, false)
                .await?;
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
