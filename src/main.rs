use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "odin")]
#[command(about = "A CLI tool for project management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure GitHub authentication
    Config,
    /// Create a new project with local folder and GitHub repo
    Create {
        /// Name of the project
        name: String,
    },
    /// List all projects and select one to work on
    List,
    /// Update project: add, commit, and push changes
    Update,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config => {
            commands::config::run();
        }
        Commands::Create { name } => {
            commands::create::run(&name);
        }
        Commands::List => {
            commands::list::run();
        }
        Commands::Update => {
            commands::update::run();
        }
    }
}
