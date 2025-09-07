#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("Incorrect usage");
        eprintln!("Usage: hexstack new");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "new" => {
            let (name, templates) = hexstack::parse_new_args(&args[2..]);
            hexstack::create_project(name, templates).await
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            Err(anyhow::anyhow!("Unknown command"))
        }
    }
    .unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
    });
}
