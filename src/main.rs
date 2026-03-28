mod cli;
mod minecraft;

use cli::commands;
use minecraft::{init, menu};
use std::env;

#[tokio::main]
async fn main() {
    init::setup();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        menu::start().await;
        return;
    }

    match args[1].as_str() {
        "-v" => commands::version(),
        "-ls" => commands::list_versions(),
        "-d" => {
            if args.len() < 3 { println!("Usage: -d <version>"); return; }
            commands::download(&args[2]).await; // Added .await
        }
        "-web" => commands::open_web(),
        "-auth" => commands::auth().await, // Added .await
        _ => commands::help(),
    }
}
