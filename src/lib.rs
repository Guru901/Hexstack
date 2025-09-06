mod setup;
pub mod templates;

use console::Style;
use dialoguer::{Input, MultiSelect, theme::ColorfulTheme};

use crate::setup::ProjectSetup;
use anyhow::Result;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

pub fn parse_new_args(args: &[String]) -> (Option<&String>, Option<Vec<String>>) {
    let mut name = None;
    let mut templates = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--template" => {
                if i + 1 < args.len() {
                    if args[i + 1].clone().to_lowercase() == "full" {
                        templates.push(String::from("ripress"));
                        templates.push(String::from("wynd"));
                    } else {
                        templates.push(args[i + 1].clone().to_lowercase());
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --template requires a value");
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') && name.is_none() => {
                name = Some(&args[i]);
                i += 1;
            }
            _ => {
                eprintln!("Warning: Unknown argument: {}", args[i]);
                i += 1;
            }
        }
    }

    let templates_option = if templates.is_empty() {
        None
    } else {
        Some(templates)
    };

    (name, templates_option)
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

    let options = &["ripress", "wynd"];

    let selected_components = match templates {
        Some(templates) => templates,
        None => {
            let selections = MultiSelect::with_theme(&theme)
                .with_prompt("Select the components you want (space to select, enter to confirm)")
                .items(options)
                .interact()?;

            let selected_components: Vec<String> = selections
                .into_iter()
                .map(|i| options[i].to_string())
                .collect();

            selected_components
        }
    };

    let project_setup = ProjectSetup::new(project_name, selected_components).await;
    project_setup.build().await?;

    Ok(())
}
