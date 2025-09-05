#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("Incorrect usage:");
        eprintln!("hexstack new <project-name>");
    }

    let command = args.get(1).unwrap();

    match command.as_str() {
        "new" => hexstack::create_project().await,
        _ => {
            eprintln!("Unknown command: {}", command);
            Err(anyhow::anyhow!("Unknown command"))
        }
    }
    .unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
    });
}
