pub mod create;
pub mod config;
pub mod list;
pub mod update;
pub mod upgrade;

use std::process::Command;

pub fn ensure_gh_installed() {
    let check = Command::new("which").arg("gh").output();
    if check.map(|o| o.status.success()).unwrap_or(false) {
        return; // Already installed
    }

    println!("GitHub CLI not found. Installing...");

    // Detect OS
    let os = std::env::consts::OS;
    let install_success = match os {
        "linux" => install_gh_linux(),
        "macos" => install_gh_macos(),
        "windows" => install_gh_windows(),
        _ => {
            eprintln!("Unsupported OS for automatic GitHub CLI installation: {}", os);
            false
        }
    };

    if !install_success {
        eprintln!("Failed to install GitHub CLI. Please install it manually from https://cli.github.com/");
        std::process::exit(1);
    }

    println!("GitHub CLI installed successfully.");
}

fn install_gh_linux() -> bool {
    // Assume Debian/Ubuntu for now
    let commands = vec![
        "curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg",
        "sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg",
        "echo 'deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main' | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null",
        "sudo apt update",
        "sudo apt install gh -y",
    ];

    for cmd in commands {
        let status = Command::new("sh").arg("-c").arg(cmd).status();
        if !status.map(|s| s.success()).unwrap_or(false) {
            return false;
        }
    }
    true
}

fn install_gh_macos() -> bool {
    let status = Command::new("brew").args(&["install", "gh"]).status();
    status.map(|s| s.success()).unwrap_or(false)
}

fn install_gh_windows() -> bool {
    let status = Command::new("winget").args(&["install", "--id", "GitHub.cli"]).status();
    status.map(|s| s.success()).unwrap_or(false)
}