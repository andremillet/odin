use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    path: String,
}

pub fn run() {
    let odin_dir = dirs::home_dir().unwrap().join(".odin");
    let projects_file = odin_dir.join("projects.json");

    if !projects_file.exists() {
        println!("No projects found. Create a project first with 'odin create <name>'.");
        return;
    }

    let data = fs::read_to_string(&projects_file).unwrap();
    let mut projects: Vec<Project> = serde_json::from_str(&data).unwrap_or_default();

    if projects.is_empty() {
        println!("No projects found.");
        return;
    }

    // Check for missing directories
    let mut missing: Vec<usize> = Vec::new();
    for (i, project) in projects.iter().enumerate() {
        if !std::path::Path::new(&project.path).exists() {
            missing.push(i);
        }
    }

    if !missing.is_empty() {
        println!("The following projects have missing directories:");
        for &i in &missing {
            println!("- {}: {}", projects[i].name, projects[i].path);
        }
        print!("Remove all missing projects from the list? (y/N): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("y") {
            // Remove in reverse order to maintain indices
            for &i in missing.iter().rev() {
                projects.remove(i);
            }
            // Save updated projects
            let json = serde_json::to_string_pretty(&projects).unwrap();
            if let Err(e) = fs::write(&projects_file, json) {
                eprintln!("Failed to update projects.json: {}", e);
            } else {
                println!("Removed missing projects from the list.");
            }
        }
    }

    if projects.is_empty() {
        println!("No projects found.");
        return;
    }

    let items: Vec<String> = projects.iter().map(|p| format!("{} - {}", p.name, p.path)).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project to work on")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    let selected_project = &projects[selection];
    println!("Selected: {}", selected_project.name);
    println!("To change to this directory, run: cd {}", selected_project.path);
}