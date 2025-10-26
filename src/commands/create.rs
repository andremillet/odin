use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    path: String,
}

pub fn run(project_name: &str) {
    // Check GitHub auth
    let auth_check = Command::new("gh").args(&["auth", "status"]).output();
    if !auth_check.map(|o| o.status.success()).unwrap_or(false) {
        println!("GitHub not configured. Please run 'odin config' first.");
        return;
    }

    let project_path = Path::new(project_name);
    if project_path.exists() {
        print!("Directory '{}' already exists. Remove and recreate? (y/N): ", project_name);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return;
        }
        if let Err(e) = fs::remove_dir_all(project_path) {
            eprintln!("Failed to remove directory: {}", e);
            return;
        }
    }

    // Create local directory
    if let Err(e) = fs::create_dir(project_path) {
        eprintln!("Failed to create directory {}: {}", project_name, e);
        return;
    }
    println!("Created directory: {}", project_name);

    // Get absolute path
    let abs_path = fs::canonicalize(project_path).unwrap();

    // Initialize CONFIG.toml
    let config_content = format!("[app]\nname = \"{}\"\npath = \"{}\"\n", project_name, abs_path.display());
    let config_path = project_path.join("CONFIG.toml");
    if let Err(e) = fs::write(&config_path, config_content) {
        eprintln!("Failed to write CONFIG.toml: {}", e);
        return;
    }

    // Change to the directory and initialize git
    if let Err(e) = Command::new("git")
        .args(&["init"])
        .current_dir(project_path)
        .status()
    {
        eprintln!("Failed to initialize git: {}", e);
        return;
    }

    // Create GitHub repo using gh CLI
    let output = Command::new("gh")
        .args(&["repo", "create", project_name, "--public", "--source=.", "--remote=origin", "--push"])
        .current_dir(project_path)
        .output();

    match output {
        Ok(result) if result.status.success() => {
            println!("GitHub repository created and pushed.");
        }
        Ok(result) => {
            eprintln!("Failed to create GitHub repo: {}", String::from_utf8_lossy(&result.stderr));
        }
        Err(e) => {
            eprintln!("Error running gh command: {}", e);
        }
    }

    // Add to projects.json
    let odin_dir = dirs::home_dir().unwrap().join(".odin");
    if let Err(e) = fs::create_dir_all(&odin_dir) {
        eprintln!("Failed to create ~/.odin: {}", e);
        return;
    }
    let projects_file = odin_dir.join("projects.json");
    let mut projects: Vec<Project> = if projects_file.exists() {
        let data = fs::read_to_string(&projects_file).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };
    projects.push(Project {
        name: project_name.to_string(),
        path: abs_path.display().to_string(),
    });
    let json = serde_json::to_string_pretty(&projects).unwrap();
    if let Err(e) = fs::write(&projects_file, json) {
        eprintln!("Failed to update projects.json: {}", e);
    } else {
        println!("Project added to global list.");
    }
}