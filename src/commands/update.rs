use std::io::{self, Write};
use std::process::Command;

pub fn run() {
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