use std::fs;
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
    let projects: Vec<Project> = serde_json::from_str(&data).unwrap();

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