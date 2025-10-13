use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct ComponentConfig {
    pub description: String,
}

#[derive(Debug, Clone)]
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
                    description: "An HTTP Framework with best in class developer experience"
                        .to_string(),
                },
            ),
            (
                "wynd".to_string(),
                ComponentConfig {
                    description: "An Event Driven WebSocket library".to_string(),
                },
            ),
            (
                "lume".to_string(),
                ComponentConfig {
                    description: "A simple and intuitive Query Builder inspired by Drizzle"
                        .to_string(),
                },
            ),
        ])
    }

    pub async fn load_templates() -> HashMap<String, ProjectTemplate> {
        HashMap::from([
            // Basic templates (no frontend)
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
                "lume".to_string(),
                ProjectTemplate {
                    name: "Lume Basic".to_string(),
                    github_url: "https://github.com/Guru901/lume-only".to_string(),
                },
            ),
            (
                "ripress_wynd".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd".to_string(),
                    github_url: "https://github.com/Guru901/ripress-wynd".to_string(),
                },
            ),
            (
                "ripress_lume".to_string(),
                ProjectTemplate {
                    name: "Ripress + Lume".to_string(),
                    github_url: "https://github.com/Guru901/ripress-lume".to_string(),
                },
            ),
            (
                "wynd_lume".to_string(),
                ProjectTemplate {
                    name: "Wynd + Lume".to_string(),
                    github_url: "https://github.com/Guru901/wynd-lume".to_string(),
                },
            ),
            (
                "ripress_wynd_lume".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd + Lume".to_string(),
                    github_url: "https://github.com/Guru901/ripress-wynd-lume".to_string(),
                },
            ),
            // React frontend templates
            (
                "ripress-react".to_string(),
                ProjectTemplate {
                    name: "Ripress + React".to_string(),
                    github_url: "https://github.com/Guru901/ripress-react".to_string(),
                },
            ),
            (
                "wynd-react".to_string(),
                ProjectTemplate {
                    name: "Wynd + React".to_string(),
                    github_url: "https://github.com/Guru901/wynd-react".to_string(),
                },
            ),
            (
                "ripress-wynd-react".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd + React".to_string(),
                    github_url: "https://github.com/Guru901/ripress-wynd-react".to_string(),
                },
            ),
            (
                "ripress-lume-react".to_string(),
                ProjectTemplate {
                    name: "Ripress + Lume + React".to_string(),
                    github_url: "https://github.com/Guru901/ripress-lume-react".to_string(),
                },
            ),
            (
                "wynd-lume-react".to_string(),
                ProjectTemplate {
                    name: "Wynd + Lume + React".to_string(),
                    github_url: "https://github.com/Guru901/wynd-lume-react".to_string(),
                },
            ),
            (
                "ripress-wynd-lume-react".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd + Lume + React".to_string(),
                    github_url: "https://github.com/Guru901/ripress-wynd-lume-react".to_string(),
                },
            ),
            // Svelte frontend templates
            (
                "ripress-svelte".to_string(),
                ProjectTemplate {
                    name: "Ripress + Svelte".to_string(),
                    github_url: "https://github.com/Guru901/ripress-svelte".to_string(),
                },
            ),
            (
                "wynd-svelte".to_string(),
                ProjectTemplate {
                    name: "Wynd + Svelte".to_string(),
                    github_url: "https://github.com/Guru901/wynd-svelte".to_string(),
                },
            ),
            (
                "ripress-wynd-svelte".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd + Svelte".to_string(),
                    github_url: "https://github.com/Guru901/ripress-wynd-svelte".to_string(),
                },
            ),
            (
                "ripress-lume-svelte".to_string(),
                ProjectTemplate {
                    name: "Ripress + Lume + Svelte".to_string(),
                    github_url: "https://github.com/Guru901/ripress-lume-svelte".to_string(),
                },
            ),
            (
                "wynd-lume-svelte".to_string(),
                ProjectTemplate {
                    name: "Wynd + Lume + Svelte".to_string(),
                    github_url: "https://github.com/Guru901/wynd-lume-svelte".to_string(),
                },
            ),
            (
                "ripress-wynd-lume-svelte".to_string(),
                ProjectTemplate {
                    name: "Ripress + Wynd + Lume + Svelte".to_string(),
                    github_url: "https://github.com/Guru901/ripress-wynd-lume-svelte".to_string(),
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

        // Determine if we have React frontend
        let has_react_frontend = self
            .selected_frontend
            .as_ref()
            .map_or(false, |f| f == "react");

        let has_svelte_frontend = self
            .selected_frontend
            .as_ref()
            .map_or(false, |f| f == "svelte");

        // Priority order for template selection (considering frontend)
        let template_priorities = if has_react_frontend {
            [
                ("ripress-wynd-lume-react", vec!["ripress", "wynd", "lume"]),
                ("ripress-wynd-react", vec!["ripress", "wynd"]),
                ("ripress-lume-react", vec!["ripress", "lume"]),
                ("wynd-lume-react", vec!["wynd", "lume"]),
                ("ripress-react", vec!["ripress"]),
                ("wynd-react", vec!["wynd"]),
                ("lume-react", vec!["lume"]),
            ]
        } else if has_svelte_frontend {
            [
                ("ripress-wynd-lume-svelte", vec!["ripress", "wynd", "lume"]),
                ("ripress-wynd-svelte", vec!["ripress", "wynd"]),
                ("ripress-lume-svelte", vec!["ripress", "lume"]),
                ("wynd-lume-svelte", vec!["wynd", "lume"]),
                ("ripress-svelte", vec!["ripress"]),
                ("wynd-svelte", vec!["wynd"]),
                ("lume-svelte", vec!["lume"]),
            ]
        } else {
            [
                ("ripress_wynd_lume", vec!["ripress", "wynd", "lume"]),
                ("ripress_wynd", vec!["ripress", "wynd"]),
                ("ripress_lume", vec!["ripress", "lume"]),
                ("wynd_lume", vec!["wynd", "lume"]),
                ("ripress", vec!["ripress"]),
                ("wynd", vec!["wynd"]),
                ("lume", vec!["lume"]),
            ]
        };

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
        // Validate project name and check for existing directory
        self.validate_project_name()?;
        self.check_directory_conflict()?;

        let total_steps = self.calculate_total_steps();
        let pb = self.create_progress_bar(total_steps)?;

        // Step 3: Generate main.rs from template
        if let Some(template) = self.determine_template() {
            pb.set_message(format!("📝 Generating main.rs from {}...", template.name));

            let output = Command::new("git")
                .arg("clone")
                .arg(template.github_url.as_str())
                .arg(self.name.as_str())
                .output()
                .await
                .context("Failed to execute git clone command")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!(
                    "Failed to clone template '{}': {}\n\nThis could be due to:\n- Network connectivity issues\n- Invalid template URL\n- Directory already exists\n- Git not installed\n\nTry running: git clone {} {}",
                    template.name,
                    stderr.trim(),
                    template.github_url,
                    self.name
                );
            }

            // Clean up git history and reinitialize
            self.cleanup_and_reinit_git().await?;

            pb.inc(1);
        } else {
            pb.set_message("⚠️  No specific template found, keeping default main.rs");
            pb.inc(1);
        }

        // Update Cargo dependencies inside the newly created project directory
        pb.set_message("🔄 Updating Cargo dependencies...");
        let project_path = PathBuf::from(&self.name);

        // Check if "backend" directory exists inside the project directory
        let backend_path = project_path.join("backend");
        let cargo_update_dir = if backend_path.is_dir() {
            &backend_path
        } else {
            &project_path
        };

        let output = Command::new("cargo")
            .arg("update")
            .current_dir(cargo_update_dir)
            .output()
            .await
            .context("Failed to execute cargo update")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "Failed to run 'cargo update' in '{}': {}",
                project_path.display(),
                stderr.trim()
            );
        }
        pb.inc(1);
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

    fn create_progress_bar(&self, total_steps: u64) -> Result<ProgressBar> {
        let pb = ProgressBar::new(total_steps);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .context("Failed to create progress bar template")?
                .progress_chars("#>-"),
        );
        Ok(pb)
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

        if let Some(frontend) = &self.selected_frontend {
            println!("\nFrontend: {}", frontend);
        }

        if let Some(template) = self.determine_template() {
            println!("\nTemplate used: {}", template.name);
        }
    }

    /// Validates the project name for common issues
    pub fn validate_project_name(&self) -> Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("Project name cannot be empty");
        }

        if self.name.len() > 50 {
            anyhow::bail!("Project name is too long (max 50 characters)");
        }

        // Check for invalid characters that could cause issues
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if self.name.chars().any(|c| invalid_chars.contains(&c)) {
            anyhow::bail!(
                "Project name contains invalid characters: {:?}\nValid characters: letters, numbers, hyphens, underscores, and dots",
                invalid_chars
            );
        }

        // Check if name starts with a number or special character
        if let Some(first_char) = self.name.chars().next() {
            if !first_char.is_alphabetic() && first_char != '_' {
                anyhow::bail!(
                    "Project name must start with a letter or underscore, not '{}'",
                    first_char
                );
            }
        }

        Ok(())
    }

    /// Checks if a directory with the same name already exists
    pub fn check_directory_conflict(&self) -> Result<()> {
        let project_path = PathBuf::from(&self.name);

        if project_path.exists() {
            if project_path.is_dir() {
                anyhow::bail!(
                    "Directory '{}' already exists!\n\nTo resolve this conflict, you can:\n1. Choose a different project name\n2. Remove the existing directory: rm -rf {}\n3. Use a different location for your project",
                    self.name,
                    self.name
                );
            } else {
                anyhow::bail!(
                    "A file named '{}' already exists in the current directory.\nPlease choose a different project name or remove the existing file.",
                    self.name
                );
            }
        }

        Ok(())
    }

    /// Cleans up git history and reinitializes the repository
    async fn cleanup_and_reinit_git(&self) -> Result<()> {
        let project_path = PathBuf::from(&self.name);

        // Remove .git directory
        let git_dir = project_path.join(".git");
        if git_dir.exists() {
            fs::remove_dir_all(&git_dir)
                .context("Failed to remove .git directory from cloned template")?;
        }

        // Initialize new git repository
        let output = Command::new("git")
            .arg("init")
            .current_dir(&project_path)
            .output()
            .await
            .context("Failed to execute git init")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to initialize git repository: {}", stderr.trim());
        }

        Ok(())
    }
}
