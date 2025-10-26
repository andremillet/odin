use std::io::{self, Write};
use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    body: String,
}

pub fn run() {
    crate::commands::ensure_gh_installed();

    let current_version = env!("CARGO_PKG_VERSION");

    println!("Checking for updates...");

    // Fetch latest release using gh
    let output = Command::new("gh")
        .args(&["api", "repos/andremillet/odin/releases/latest"])
        .output();

    let release: Release = match output {
        Ok(result) if result.status.success() => {
            let json = String::from_utf8_lossy(&result.stdout);
            serde_json::from_str(&json).unwrap_or_else(|_| {
                eprintln!("Failed to parse release info.");
                std::process::exit(1);
            })
        }
        _ => {
            eprintln!("Failed to fetch latest release. Ensure GitHub CLI is configured.");
            return;
        }
    };

    let latest_version = release.tag_name.trim_start_matches('v');

    if latest_version == current_version {
        println!("Odin is up to date (v{}).", current_version);
        return;
    }

    println!("A new version is available: v{} (current: v{})", latest_version, current_version);
    println!("\nRelease notes:\n{}", release.body);

    print!("Do you want to install the update? (y/N): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Update cancelled.");
        return;
    }

    // Download and install
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let asset_name = format!("odin-{}-{}", os, arch);

    let download_url = format!("https://github.com/andremillet/odin/releases/latest/download/{}", asset_name);

    println!("Downloading {}...", asset_name);

    let temp_path = "/tmp/odin_new";
    let download = Command::new("curl")
        .args(&["-L", "-o", temp_path, &download_url])
        .status();

    if !download.map(|s| s.success()).unwrap_or(false) {
        eprintln!("Failed to download the update.");
        return;
    }

    // Find current binary path
    let which = Command::new("which").arg("odin").output();
    let binary_path = match which {
        Ok(result) if result.status.success() => {
            String::from_utf8_lossy(&result.stdout).trim().to_string()
        }
        _ => {
            eprintln!("Could not find odin binary path.");
            return;
        }
    };

    // Replace binary (may need sudo)
    let mv = Command::new("sudo")
        .args(&["mv", temp_path, &binary_path])
        .status();

    if mv.map(|s| s.success()).unwrap_or(false) {
        println!("Odin updated to v{} successfully!", latest_version);
    } else {
        eprintln!("Failed to install the update. You may need to run with sudo.");
    }
}