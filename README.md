# Odin

Odin is a CLI tool for managing software projects, serving as a wrapper around GitHub CLI. It simplifies project creation, configuration, and management by automating Git and GitHub operations.

## Features

- **Config**: Authenticate with GitHub using `gh auth login`.
- **Create**: Create a new project with a local directory, initialize Git, and set up a GitHub repository.
- **List**: Interactively list and select projects to work on, providing the path to change directories.
- **Update**: (Placeholder for future update functionality, e.g., add, commit, and push changes).

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/andremillet/odin.git
   cd odin
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install globally (optional):
   ```bash
   cargo install --path .
   ```

Or use the provided install script:
```bash
./install.sh
```

## Usage

### Configure GitHub
```bash
odin config
```
This runs `gh auth login` to authenticate with GitHub.

### Create a New Project
```bash
odin create <project-name>
```
- Checks for GitHub authentication.
- Creates a local directory (prompts if it exists).
- Initializes a Git repository.
- Creates a `CONFIG.toml` with project details.
- Sets up a GitHub repository and pushes the initial commit.
- Updates the global project list in `~/.odin/projects.json`.

### List Projects
```bash
odin list
```
Displays an interactive menu to select a project. After selection, it provides the command to change to the project directory.

### Update Project (Future)
```bash
odin update
```
Placeholder for committing and pushing changes.

## Requirements

- Rust (for building)
- GitHub CLI (`gh`) installed and authenticated
- Git

## Project Structure

- `src/main.rs`: Entry point and CLI definition.
- `src/commands/`: Individual command implementations.
- `Cargo.toml`: Dependencies and project metadata.
- `install.sh`: Installation script.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is open-source. Check the license file for details.