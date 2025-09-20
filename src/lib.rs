mod setup;

use console::Style;
use dialoguer::{Input, MultiSelect, Select, theme::ColorfulTheme};
use serde_json::Value;
use tokio::process::Command as AsyncCommand;

use crate::setup::ProjectSetup;
use anyhow::Result;

#[cfg(test)]
mod tests;

pub fn parse_new_args(args: &[String]) -> Result<(Option<&String>, Option<Vec<String>>)> {
    let mut name = None;
    let mut templates = Vec::new();
    let mut i = 0;
    let mut errors = Vec::new();

    while i < args.len() {
        match args[i].as_str() {
            "--template" => {
                if i + 1 < args.len() {
                    let template_value = args[i + 1].clone().to_lowercase();
                    match template_value.as_str() {
                        "full" => {
                            templates.push(String::from("ripress"));
                            templates.push(String::from("wynd"));
                        }
                        "ripress" | "wynd" => {
                            templates.push(template_value);
                        }
                        _ => {
                            errors.push(format!(
                                "Invalid template value '{}'. Valid values: full, ripress, wynd",
                                args[i + 1]
                            ));
                        }
                    }
                    i += 2;
                } else {
                    errors.push("--template requires a value".to_string());
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') && name.is_none() => {
                // Validate project name early
                if arg.is_empty() {
                    errors.push("Project name cannot be empty".to_string());
                } else if arg.len() > 50 {
                    errors.push("Project name is too long (max 50 characters)".to_string());
                } else {
                    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
                    if arg.chars().any(|c| invalid_chars.contains(&c)) {
                        errors.push(format!(
                            "Project name '{}' contains invalid characters. Valid characters: letters, numbers, hyphens, underscores, and dots",
                            arg
                        ));
                    } else if let Some(first_char) = arg.chars().next() {
                        if !first_char.is_alphabetic() && first_char != '_' {
                            errors.push(format!(
                                "Project name must start with a letter or underscore, not '{}'",
                                first_char
                            ));
                        }
                    }
                }
                name = Some(&args[i]);
                i += 1;
            }
            _ => {
                errors.push(format!("Unknown argument: {}", args[i]));
                i += 1;
            }
        }
    }

    if !errors.is_empty() {
        anyhow::bail!("Argument parsing errors:\n{}", errors.join("\n"));
    }

    let templates_option = if templates.is_empty() {
        None
    } else {
        Some(templates)
    };

    Ok((name, templates_option))
}

pub async fn create_project(
    project_name: Option<&String>,
    templates: Option<Vec<String>>,
) -> Result<()> {
    let dull = Style::new().dim();
    let underline = Style::new().underlined();
    let theme = ColorfulTheme {
        prompt_style: Style::new().cyan().bold(),
        prompt_suffix: dull.apply_to(" ›".to_string()),
        defaults_style: underline.cyan(),
        values_style: Style::new().green(),
        active_item_style: Style::new().cyan().bold(),
        inactive_item_style: dull.clone(),
        picked_item_prefix: Style::new().green().apply_to("✔".to_string()),
        unpicked_item_prefix: dull.apply_to("•".to_string()),
        ..ColorfulTheme::default()
    };

    let project_name: String = match project_name {
        Some(name) => name.clone(),
        None => {
            let project_name: String = Input::with_theme(&theme)
                .with_prompt("What should the name of your project be")
                .default("my-app".into())
                .interact_text()?;
            project_name
        }
    };

    println!("📦 Creating project `{}`", project_name);

    let component_options = &["ripress", "wynd"];

    let selected_components = match templates {
        Some(templates) => templates,
        None => {
            let selections = MultiSelect::with_theme(&theme)
                .with_prompt("Select the components you want (space to select, enter to confirm)")
                .items(component_options.iter().map(|f| capitalize(f)))
                .interact()?;

            let selected_components: Vec<String> = selections
                .into_iter()
                .map(|i| component_options[i].to_string())
                .collect();

            selected_components
        }
    };

    let frontend_options = vec!["react", "svelte", "none"];

    let selection = Select::with_theme(&theme)
        .with_prompt("Select the frontend you want")
        .items(frontend_options.clone().into_iter().map(|f| capitalize(f)))
        .interact()?;

    let selected_frontend = frontend_options[selection];

    if selected_frontend == "none" {
        println!("🚧 Creating project `{}` without frontend", project_name);
    } else {
        println!("🚧 Creating project `{}` with frontend", project_name);
    }

    let selected_frontend = match selected_frontend {
        "react" => Some(String::from("react")),
        "svelte" => Some(String::from("svelte")),
        "none" => None,
        _ => {
            eprintln!("Error: Invalid frontend");
            None
        }
    };

    let project_setup =
        ProjectSetup::new(project_name, selected_components, selected_frontend).await;
    project_setup.build().await?;

    Ok(())
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub async fn update_if_needed() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let latest_version = get_latest_version().await?;

    if latest_version != version {
        println!(
            "A new version of hexstack is available ({} → {})",
            version, latest_version
        );
        println!("Updating...");

        let output = AsyncCommand::new("cargo")
            .args(&["install", "hexstack"])
            .output()
            .await?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to update: {}", error));
        }

        println!("Updated hexstack to the latest version!");
        println!("Please restart hexstack to use the new version.");

        // Instead of running the old binary, suggest restart
        // or use std::process::exit(0) to terminate current process
    }
    Ok(())
}

async fn get_latest_version() -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .user_agent(format!(
            "hexstack/{} (+https://github.com/guru901/hexstack)",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build HTTP client: {}", e))?;

    let response = client
        .get("https://crates.io/api/v1/crates/hexstack")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch crate info: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "API request failed with status: {}",
            response.status()
        ));
    }

    let json: Value = response
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON response: {}", e))?;

    let latest_version = json
        .get("crate")
        .and_then(|c| {
            c.get("max_stable_version")
                .or_else(|| c.get("max_version"))
                .or_else(|| c.get("newest_version"))
        })
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid API response format"))?;

    Ok(latest_version.to_string())
}
