#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("Incorrect usage");
        eprintln!("Usage: hexstack new [project-name] [--template <template>]");
        eprintln!("\nExamples:");
        eprintln!("  hexstack new my-app");
        eprintln!("  hexstack new my-app --template full");
        eprintln!("  hexstack new my-app --template ripress");
        eprintln!("  hexstack new my-app --template wynd");
        eprintln!("  hexstack new my-app --template lume");
        return;
    }

    let command = &args[1];

    if let Err(e) = hexstack::update_if_needed().await {
        eprintln!("Auto-update check failed: {e}");
        eprintln!("Continuing without updating. To update manually, run: cargo install hexstack");
    }

    let result = match command.as_str() {
        "new" => match hexstack::parse_new_args(&args[2..]) {
            Ok((name, templates)) => hexstack::create_project(name, templates).await,
            Err(e) => Err(e),
        },
        "--version" => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        _ => Err(anyhow::anyhow!(
            "Unknown command: {}\n\nAvailable commands:\n  new    Create a new project",
            command
        )),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
