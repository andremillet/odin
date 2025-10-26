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
    let config_exists = Path::new("CONFIG.toml").exists();
    let odin_dir = dirs::home_dir().unwrap().join(".odin");
    let projects_file = odin_dir.join("projects.json");

    let current_path = std::env::current_dir().unwrap().canonicalize().unwrap();
    let current_path_str = current_path.display().to_string();

    if !config_exists {
        print!("This directory is not an Odin project. Would you like to add it? (y/N): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            eprintln!("Not in an Odin project directory. Aborting.");
            return;
        }

        // Prompt for project name
        print!("Enter project name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();
        if name.is_empty() {
            eprintln!("Project name cannot be empty.");
            return;
        }

        // Create CONFIG.toml
        let config_content = format!("[app]\nname = \"{}\"\npath = \"{}\"\n", name, current_path_str);
        if let Err(e) = fs::write("CONFIG.toml", config_content) {
            eprintln!("Failed to create CONFIG.toml: {}", e);
            return;
        }

        // Add to projects.json
        if let Err(e) = fs::create_dir_all(&odin_dir) {
            eprintln!("Failed to create ~/.odin: {}", e);
            return;
        }
        let mut projects: Vec<serde_json::Value> = if projects_file.exists() {
            let data = fs::read_to_string(&projects_file).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Vec::new()
        };
        let new_project = serde_json::json!({
            "name": name,
            "path": current_path_str
        });
        projects.push(new_project);
        let json = serde_json::to_string_pretty(&projects).unwrap();
        if let Err(e) = fs::write(&projects_file, json) {
            eprintln!("Failed to update projects.json: {}", e);
            return;
        }

        println!("Project '{}' added to Odin management.", name);
    } else {
        // Validate the project is in projects.json
        if projects_file.exists() {
            let data = fs::read_to_string(&projects_file).unwrap_or_default();
            let projects: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap_or_default();
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