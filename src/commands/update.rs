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

        // After adoption, offer to create GitHub repo if no remote
        let remote_check = Command::new("git")
            .args(&["remote", "get-url", "origin"])
            .output();

        if remote_check.is_err() {
            print!("No remote repository found. Would you like to create a GitHub repository and push? (y/N): ");
            io::stdout().flush().unwrap();
            let mut create_remote = String::new();
            io::stdin().read_line(&mut create_remote).unwrap();
            if create_remote.trim().eq_ignore_ascii_case("y") {
                crate::commands::ensure_gh_installed();

                // Check GitHub auth
                let auth_check = Command::new("gh").args(&["auth", "status"]).output();
                if !auth_check.map_or(false, |o| o.status.success()) {
                    eprintln!("GitHub not authenticated. Run 'odin config' first.");
                    return;
                }

                println!("Creating GitHub repository...");
                // Create GitHub repo
                let output = Command::new("gh")
                    .args(&["repo", "create", &name, "--public", "--source=.", "--remote=origin", "--push"])
                    .current_dir(".")
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
            }
        }
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

    println!("Checking for uncommitted changes...");
    let status_output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output();

    let has_changes = match status_output {
        Ok(output) => !output.stdout.is_empty(),
        Err(e) => {
            eprintln!("Failed to check git status: {}", e);
            return;
        }
    };

    if !has_changes {
        println!("No uncommitted changes found.");
    } else {
        println!("Found uncommitted changes.");
        println!("Staging changes...");
        // Git add all changes
        let add_status = Command::new("git")
            .args(&["add", "."])
            .status();

        if !add_status.map_or(false, |s| s.success()) {
            eprintln!("Failed to stage files.");
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

        println!("Committing changes...");
        // Git commit
        let commit_status = Command::new("git")
            .args(&["commit", "-m", message])
            .status();

        if !commit_status.map_or(false, |s| s.success()) {
            eprintln!("Failed to commit changes.");
            return;
        }
        println!("Committed with message: {}", message);
    }

    // Check if there's a remote before pushing
    let remote_check = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .output();

    if let Ok(output) = remote_check {
        if output.status.success() {
            let remote_url = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Get project name from CONFIG.toml
        let config_content = fs::read_to_string("CONFIG.toml").unwrap_or_default();
        let project_name = if let Some(line) = config_content.lines().find(|l| l.starts_with("name = ")) {
            line.split('"').nth(1).unwrap_or("Unknown").to_string()
        } else {
            "Unknown".to_string()
        };

        println!("Project: {}", project_name);
        println!("Remote: {}", remote_url);
        print!("Proceed with push? (y/N): ");
        io::stdout().flush().unwrap();
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).unwrap();
        if !confirm.trim().eq_ignore_ascii_case("y") {
            println!("Push cancelled.");
            return;
        }

        println!("Pushing to remote...");
        // Git push
        let push_status = Command::new("git")
            .args(&["push"])
            .status();

        if push_status.map_or(false, |s| s.success()) {
            println!("Pushed to remote repository.");
        } else {
            eprintln!("Failed to push to remote.");
        }
}

    println!("Checking remote synchronization...");
    // Check if remote exists
    let remotes_output = Command::new("git")
        .args(&["remote"])
        .output();

    let has_remote = remotes_output.ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("origin"))
        .unwrap_or(false);

    if has_remote {
        let remote_check = Command::new("git")
            .args(&["remote", "get-url", "origin"])
            .output();

        if let Ok(output) = remote_check {
            if output.status.success() {
                let _remote_url = String::from_utf8_lossy(&output.stdout).trim().to_string();

                println!("Fetching from remote...");
                // Fetch remote
                let fetch_status = Command::new("git")
                    .args(&["fetch", "origin"])
                    .status();

                if fetch_status.map_or(false, |s| s.success()) {
                // Check if ahead or behind
                let ahead_output = Command::new("git")
                    .args(&["rev-list", "--count", "HEAD..origin/master"])
                    .output();

                let behind_output = Command::new("git")
                    .args(&["rev-list", "--count", "origin/master..HEAD"])
                    .output();

                let ahead = ahead_output.ok()
                    .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse::<i32>().ok())
                    .unwrap_or(0);

                let behind = behind_output.ok()
                    .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse::<i32>().ok())
                    .unwrap_or(0);

                if ahead > 0 && behind == 0 {
                    print!("Local is {} commits ahead. Push to remote? (y/N): ", ahead);
                    io::stdout().flush().unwrap();
                    let mut push_confirm = String::new();
                    io::stdin().read_line(&mut push_confirm).unwrap();
                    if push_confirm.trim().eq_ignore_ascii_case("y") {
                        println!("Pushing to remote...");
                        let push_status = Command::new("git")
                            .args(&["push"])
                            .status();
                        if push_status.map_or(false, |s| s.success()) {
                            println!("Pushed to remote.");
                        } else {
                            eprintln!("Failed to push to remote.");
                        }
                    }
                } else if behind > 0 && ahead == 0 {
                    print!("Local is {} commits behind. Pull from remote? (y/N): ", behind);
                    io::stdout().flush().unwrap();
                    let mut pull_confirm = String::new();
                    io::stdin().read_line(&mut pull_confirm).unwrap();
                    if pull_confirm.trim().eq_ignore_ascii_case("y") {
                        println!("Pulling from remote...");
                        let pull_status = Command::new("git")
                            .args(&["pull"])
                            .status();
                        if pull_status.map_or(false, |s| s.success()) {
                            println!("Pulled from remote.");
                        } else {
                            eprintln!("Failed to pull from remote.");
                        }
                    }
                } else if ahead > 0 && behind > 0 {
                    println!("Local and remote have diverged ({} ahead, {} behind). Manual resolution needed.", ahead, behind);
                    } else {
                        println!("Local repository is up to date with remote.");
                    }
                } else {
                    eprintln!("Failed to fetch from remote.");
                }
            }
        }
    } else {
        println!("No remote repository configured.");
        print!("Would you like to create a GitHub repository? (y/N): ");
        io::stdout().flush().unwrap();
        let mut create_confirm = String::new();
        io::stdin().read_line(&mut create_confirm).unwrap();
        if create_confirm.trim().eq_ignore_ascii_case("y") {
            // Get project name from CONFIG.toml
            let config_content = fs::read_to_string("CONFIG.toml").unwrap_or_default();
            let project_name = if let Some(line) = config_content.lines().find(|l| l.starts_with("name = ")) {
                line.split('"').nth(1).unwrap_or("unknown").to_string()
            } else {
                "unknown".to_string()
            };

            crate::commands::ensure_gh_installed();

            // Check GitHub auth
            let auth_check = Command::new("gh").args(&["auth", "status"]).output();
            if !auth_check.map_or(false, |o| o.status.success()) {
                eprintln!("GitHub not authenticated. Run 'odin config' first.");
                return;
            }

            println!("Creating GitHub repository...");
            // Create GitHub repo
            let output = Command::new("gh")
                .args(&["repo", "create", &project_name, "--public", "--source=.", "--remote=origin", "--push"])
                .current_dir(".")
                .output();

            match output {
                Ok(result) if result.status.success() => {
                    println!("GitHub repository created and pushed.");
                }
                Ok(result) => {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    if stderr.contains("Name already exists") {
                        print!("Repository '{}' already exists. (n)ew name, (o)verwrite, (c)ancel? ", project_name);
                        io::stdout().flush().unwrap();
                        let mut choice = String::new();
                        io::stdin().read_line(&mut choice).unwrap();
                        let choice = choice.trim().to_lowercase();
                        match choice.as_str() {
                            "n" => {
                                print!("Enter new repository name: ");
                                io::stdout().flush().unwrap();
                                let mut new_name = String::new();
                                io::stdin().read_line(&mut new_name).unwrap();
                                let new_name = new_name.trim();
                                if new_name.is_empty() {
                                    eprintln!("Name cannot be empty. Cancelled.");
                                    return;
                                }
                                // Retry with new name
                                let output2 = Command::new("gh")
                                    .args(&["repo", "create", &new_name, "--public", "--source=.", "--remote=origin", "--push"])
                                    .current_dir(".")
                                    .output();
                                match output2 {
                                    Ok(r) if r.status.success() => {
                                        println!("GitHub repository '{}' created and pushed.", new_name);
                                    }
                                    _ => {
                                        eprintln!("Failed to create repository with new name.");
                                    }
                                }
                            }
                            "o" => {
                                println!("Deleting existing repository...");
                                let delete_output = Command::new("gh")
                                    .args(&["repo", "delete", &project_name, "--yes"])
                                    .output();
                                if let Ok(del_out) = delete_output {
                                    if del_out.status.success() {
                                        println!("Recreating repository...");
                                        let output3 = Command::new("gh")
                                            .args(&["repo", "create", &project_name, "--public", "--source=.", "--remote=origin", "--push"])
                                            .current_dir(".")
                                            .output();
                                        match output3 {
                                            Ok(r) if r.status.success() => {
                                                println!("GitHub repository recreated and pushed.");
                                            }
                                            _ => {
                                                eprintln!("Failed to recreate repository.");
                                            }
                                        }
                                    } else {
                                        let stderr = String::from_utf8_lossy(&del_out.stderr);
                                        eprintln!("Failed to delete existing repository: {}", stderr);
                                        if stderr.contains("delete_repo") {
                                            println!("Refreshing auth to add delete_repo scope...");
                                            let refresh_status = Command::new("gh")
                                                .args(&["auth", "refresh", "-h", "github.com", "-s", "delete_repo"])
                                                .status();
                                            if refresh_status.map_or(false, |s| s.success()) {
                                                println!("Auth refreshed. Retrying deletion...");
                                                // Retry delete
                                                let retry_delete = Command::new("gh")
                                                    .args(&["repo", "delete", &project_name, "--yes"])
                                                    .output();
                                                if let Ok(retry_out) = retry_delete {
                                                    if retry_out.status.success() {
                                                        println!("Recreating repository...");
                                                        let output3 = Command::new("gh")
                                                            .args(&["repo", "create", &project_name, "--public", "--source=.", "--remote=origin", "--push"])
                                                            .current_dir(".")
                                                            .output();
                                                        match output3 {
                                                            Ok(r) if r.status.success() => {
                                                                println!("GitHub repository recreated and pushed.");
                                                            }
                                                            _ => {
                                                                eprintln!("Failed to recreate repository.");
                                                            }
                                                        }
                                                    } else {
                                                        let retry_stderr = String::from_utf8_lossy(&retry_out.stderr);
                                                        eprintln!("Retry delete failed: {}", retry_stderr);
                                                    }
                                                } else {
                                                    eprintln!("Failed to retry delete.");
                                                }
                                            } else {
                                                eprintln!("Auth refresh failed. Run manually: gh auth refresh -h github.com -s delete_repo");
                                            }
                                        }
                                    }
                                } else {
                                    eprintln!("Failed to run delete command.");
                                }
                            }
                            _ => {
                                println!("Cancelled.");
                            }
                        }
                    } else {
                        eprintln!("Failed to create GitHub repo: {}", stderr);
                    }
                }
                Err(e) => {
                    eprintln!("Error running gh command: {}", e);
                }
            }
        }
    }
    } else {
        println!("No remote repository configured. Skipping push.");
    }
}