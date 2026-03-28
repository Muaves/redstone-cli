use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<VersionSummary>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Latest { pub release: String, pub snapshot: String }

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionSummary { pub id: String, pub r#type: String, pub url: String }

pub async fn fetch_versions() -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    let resp = reqwest::get(url).await?.json::<VersionManifest>().await?;
    Ok(resp)
}

pub async fn fetch_version_details(url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    Ok(resp)
}

pub async fn download_version(json: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let id = json["id"].as_str().unwrap_or("unknown");
    let path = format!("launcher_data/versions/{}.json", id);
    fs::write(&path, serde_json::to_string_pretty(json)?)?;
    println!("\x1b[92m[Success]\x1b[0m Metadata saved to: {}", path);
    Ok(())
}
