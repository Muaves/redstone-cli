use std::process::Command;
use std::path::Path;

pub fn launch(version: &str) -> Result<(), String> {
    let jar_path = format!("launcher_data/versions/{}.jar", version);
    let game_dir = "launcher_data";

    if !Path::new(&jar_path).exists() {
        return Err(format!("Version {} not found. Please download it first (Option 2).", version));
    }

    println!("\x1b[92m[Launcher]\x1b[0m Starting Minecraft {}...", version);

    let status = Command::new("java")
        .arg("-Xmx2G")
        .arg("-cp")
        .arg(&jar_path)
        .arg("net.minecraft.client.main.Main")
        .arg("--version")
        .arg(version)
        .arg("--gameDir")
        .arg(game_dir)
        .arg("--assetsDir")
        .arg(format!("{}/assets", game_dir))
        .spawn();

    match status {
        Ok(_) => {
            println!("\x1b[92m[Success]\x1b[0m Game process started.");
            Ok(())
        }
        Err(e) => Err(format!("Failed to start Java: {}", e)),
    }
}