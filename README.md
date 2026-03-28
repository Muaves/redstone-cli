# Redstone CLI
A high-performance, Rust-based Minecraft Launcher. Redstone CLI eliminates the need for heavy, memory-intensive graphical launchers by managing version manifests, downloads, and authentication directly through the terminal.

## Features
* Interactive Menu: Simplified navigation using a numeric interface (Options 1-6).
* Microsoft Auth: Secure implementation of the OAuth2 Device Code flow.
* Smart Downloader: Concurrent fetching of both version metadata (JSON) and client game files (JAR).
* Clean & Portable: Self-contained architecture that stores data locally without system-wide clutter.

## Installation and Execution

### For Users (Binary)
1. Navigate to the [Releases](https://github.com/Muaves/redstone-cli/releases) page.
2. Download the binary compatible with your operating system.
3. Execute the file: `./redstone-cli`

### For Developers (From Source)
To build the project manually, ensure the Rust toolchain is installed:
```bash
git clone [https://github.com/Muaves/redstone-cli.git](https://github.com/Muaves/redstone-cli.git)
cd "Redstone CLI"
cargo run
