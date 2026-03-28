use std::io::{self, Write};
use crate::minecraft::versions;
use crate::minecraft::launcher;
use crate::cli::commands;

pub async fn start() {
    loop {
        println!("\n\x1b[91m=== REDSTONE CLI ===\x1b[0m");
        println!("1. List Latest Versions");
        println!("2. Download a Version");
        println!("3. Launch Minecraft");
        println!("4. Open Website");
        println!("5. Authentication");
        println!("6. Exit");
        print!("\x1b[93mSelect option:\x1b[0m ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => {
                println!("Fetching...");
                if let Ok(m) = versions::fetch_versions().await {
                    println!("Latest Release: {}", m.latest.release);
                    for v in m.versions.iter().take(5) {
                        println!(" - {} ({})", v.id, v.r#type);
                    }
                }
            },
            "2" => {
                print!("Enter version id (e.g., 1.20.1): ");
                io::stdout().flush().unwrap();
                let mut v = String::new();
                io::stdin().read_line(&mut v).unwrap();
                commands::download(v.trim()).await; // Added .await
            },
            "3" => {
                print!("Version to launch: ");
                io::stdout().flush().unwrap();
                let mut v = String::new();
                io::stdin().read_line(&mut v).unwrap();
                let _ = launcher::launch(v.trim());
            },
            "4" => commands::open_web(),
            "5" => commands::auth().await, // Added .await
            "6" => break,
            _ => println!("Invalid choice!"),
        }
    }
}
