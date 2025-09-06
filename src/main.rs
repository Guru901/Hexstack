#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("Usage: hexstack new");
        return;
    }

    let command = &args[1];
    let mut name = None;

    if command == "new" {
        name = args.get(2);
    }

    match command.as_str() {
        "new" => hexstack::create_project(name).await,
        _ => {
            eprintln!("Unknown command: {}", command);
            Err(anyhow::anyhow!("Unknown command"))
        }
    }
    .unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
    });
}
