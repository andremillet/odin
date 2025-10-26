use std::process::Command;

pub fn run() {
    crate::commands::ensure_gh_installed();

    println!("Starting GitHub authentication...");
    println!("Follow the prompts to authenticate with your GitHub account.");
    println!("After completing authentication in the browser, the command should exit automatically.");

    // Run gh auth login to authenticate with GitHub
    let status = Command::new("gh")
        .args(&["auth", "login"])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("GitHub authentication configured successfully.");
        }
        Ok(_s) => {
            eprintln!("GitHub authentication failed or was cancelled.");
        }
        Err(e) => {
            eprintln!("Error running gh auth login: {}", e);
        }
    }
}