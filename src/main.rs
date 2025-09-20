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
        return;
    }

    let command = &args[1];

    if let Err(e) = hexstack::update_if_needed().await {
        println!("Auto Update failed: {}", e);
        println!("Please run `cargo install hexstack` to install the latest version");
        std::process::exit(1);
    }

    let result = match command.as_str() {
        "new" => match hexstack::parse_new_args(&args[2..]) {
            Ok((name, templates)) => hexstack::create_project(name, templates).await,
            Err(e) => Err(e),
        },
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
