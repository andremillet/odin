use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn run() {
    // Check if current directory is a git repo
    if !Path::new(".git").exists() {
        eprintln!("Not in a Git repository. Initialize with 'git init' or navigate to a project.");
        return;
    }

    // Check for CONFIG.toml
    if !Path::new("CONFIG.toml").exists() {
        eprintln!("Not in an Odin project directory. CONFIG.toml not found.");
        return;
    }

    // Optionally, validate the project is in projects.json
    let odin_dir = dirs::home_dir().unwrap().join(".odin");
    let projects_file = odin_dir.join("projects.json");
    if projects_file.exists() {
        let data = fs::read_to_string(&projects_file).unwrap_or_default();
        let projects: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap_or_default();
        let current_path = std::env::current_dir().unwrap().canonicalize().unwrap();
        let is_tracked = projects.iter().any(|p| {
            if let Some(path) = p.get("path") {
                if let Some(path_str) = path.as_str() {
                    Path::new(path_str).canonicalize().ok() == Some(current_path.clone())
                } else {
                    false
                }
            } else {
                false
            }
        });
        if !is_tracked {
            eprintln!("Current directory is not tracked as an Odin project.");
            return;
        }
    }

    // Git add all changes
    let add_status = Command::new("git")
        .args(&["add", "."])
        .status();

    if !add_status.map_or(false, |s| s.success()) {
        eprintln!("Failed to add files.");
        return;
    }
    println!("Staged all changes.");

    // Prompt for commit message
    print!("Enter commit message: ");
    io::stdout().flush().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();

    if message.is_empty() {
        eprintln!("Commit message cannot be empty.");
        return;
    }

    // Git commit
    let commit_status = Command::new("git")
        .args(&["commit", "-m", message])
        .status();

    if !commit_status.map_or(false, |s| s.success()) {
        eprintln!("Failed to commit.");
        return;
    }
    println!("Committed with message: {}", message);

    // Git push
    let push_status = Command::new("git")
        .args(&["push"])
        .status();

    if push_status.map_or(false, |s| s.success()) {
        println!("Pushed to remote repository.");
    } else {
        eprintln!("Failed to push.");
    }
}