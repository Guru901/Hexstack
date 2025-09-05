use anyhow::Result;
use console::Style;
use dialoguer::{Input, MultiSelect, theme::ColorfulTheme};
use hexstack::ProjectSetup;

#[tokio::main]
async fn main() -> Result<()> {
    // Styles for the CLI
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

    // Project name input
    let project_name: String = Input::with_theme(&theme)
        .with_prompt("What should the name of your project be")
        .default("my-app".into())
        .interact_text()
        .unwrap();

    println!("📦 Creating project `{}`", project_name);

    // Component selection
    let options = &["Ripress", "Wynd"];
    let selections = MultiSelect::with_theme(&theme)
        .with_prompt("Select the components you want (space to select, enter to confirm)")
        .items(options)
        .interact()
        .unwrap();

    let selected_components: Vec<String> = selections
        .into_iter()
        .map(|i| options[i].to_string())
        .collect();

    // Create and build project
    let project_setup = ProjectSetup::new(project_name, selected_components).await;
    project_setup.build().await?;

    Ok(())
}
