use std::io::{self, Write};
use crate::minecraft::versions;
use crate::minecraft::launcher;

pub async fn start() {
    loop {
        println!("\n=== Redstone CLI ===");
        println!("1. List Versions");
        println!("2. Download Version");
        println!("3. Launch Version");
        println!("4. Exit");

        print!("Select: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => {
                let manifest = versions::fetch_versions().await.unwrap();
                println!("\nLatest release: {}", manifest.latest.release);
                println!("Latest snapshot: {}", manifest.latest.snapshot);
                for v in manifest.versions.iter().take(10) {
                    println!("{} ({})", v.id, v.r#type);
                }
            }
            "2" => {
                let manifest = versions::fetch_versions().await.unwrap();
                println!("Enter version id to download:");
                let mut ver_input = String::new();
                io::stdin().read_line(&mut ver_input).unwrap();
                let ver_input = ver_input.trim();
                if let Some(summary) = manifest.versions.iter().find(|x| x.id == ver_input) {
                    let version = versions::fetch_version_details(&summary.url).await.unwrap();
                    versions::download_version(&version).await.unwrap();
                } else {
                    println!("Version not found!");
                }
            }
            "3" => {
                println!("Enter version id to launch:");
                let mut ver_input = String::new();
                io::stdin().read_line(&mut ver_input).unwrap();
                if let Err(e) = launcher::launch(ver_input.trim()) {
                    println!("Error launching: {}", e);
                }
            }
            "4" => break,
            _ => println!("Invalid option"),
        }
    }
}
