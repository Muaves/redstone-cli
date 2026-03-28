use std::process::Command;
use crate::minecraft::versions;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;

pub fn version() {
    println!("\x1b[95m[Redstone CLI]\x1b[0m Version 0.1.0");
}

pub fn help() {
    println!("Usage: cargo run -- [option]\n-v : Version\n-ls: List\n-r : Launch");
}

pub fn open_web() {
    println!("\x1b[94m[Web]\x1b[0m Opening Minecraft.net...");
    let _ = Command::new("xdg-open").arg("https://www.minecraft.net").spawn();
}

pub async fn auth() {
    println!("\x1b[93m[Auth]\x1b[0m Requesting real code from Microsoft...");
    
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));

    // We switch to the Live.com login system which is more reliable for Minecraft
    let url = "https://login.live.com/oauth20_connect.srf";
    
    // This is the Client ID for the Minecraft Launcher on Windows
    let params = [
        ("client_id", "00000000402b5328"),
        ("scope", "service::user.auth.xboxlive.com::MBI_SSL"),
        ("response_type", "device_code"),
    ];

    let res = client.post(url)
        .headers(headers)
        .form(&params)
        .send()
        .await;

    match res {
        Ok(response) => {
            let text = response.text().await.unwrap_or_default();
            let json: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
            
            if let Some(user_code) = json["user_code"].as_str() {
                println!("\n\x1b[1;92mAUTHENTICATION REQUIRED:\x1b[0m");
                println!("1. Open: \x1b[4;94mhttps://www.microsoft.com/link\x1b[0m");
                println!("2. Enter Code: \x1b[1;91m{}\x1b[0m", user_code);
                println!("\nWaiting for your authorization...");
            } else {
                println!("\x1b[91m[Error]\x1b[0m Authentication failed.");
                println!("Microsoft Response: {}", text);
            }
        }
        Err(e) => println!("\x1b[91m[Error]\x1b[0m Connection failed: {}", e),
    }
}

pub async fn download(version_id: &str) {
    println!("\x1b[92m[Downloader]\x1b[0m Searching for {}...", version_id);
    if let Ok(manifest) = versions::fetch_versions().await {
        if let Some(summary) = manifest.versions.iter().find(|v| v.id == version_id) {
            if let Ok(details) = versions::fetch_version_details(&summary.url).await {
                let _ = versions::download_version(&details).await;
            }
        }
    }
}

pub fn list_versions() { println!("Use option 1 in the menu."); }
