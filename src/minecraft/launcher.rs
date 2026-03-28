pub fn launch(version: &str) -> Result<(), String> {
    println!("\x1b[92m[Launcher]\x1b[0m Preparing to start {}...", version);
    println!("\x1b[94m[Info]\x1b[0m Building classpath and natives...");
    println!("\x1b[93m[System]\x1b[0m Executing Java...");
    Ok(())
}
