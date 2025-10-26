# Odin

Odin is a CLI tool for managing software projects, serving as a wrapper around GitHub CLI. It simplifies project creation, configuration, and management by automating Git and GitHub operations. GitHub CLI is installed automatically if not present.

## Features

- **Config**: Authenticate with GitHub using `gh auth login`.
- **Create**: Create a new project with a local directory, initialize Git, and set up a GitHub repository.
- **List**: Interactively list and select projects to work on, providing the path to change directories.
- **Update**: Add, commit, and push changes to the current project.
- **Upgrade**: Check for Odin updates on GitHub and install them if desired.

## Quick Install

For a quick installation without cloning the repository:

```bash
curl -fsSL https://raw.githubusercontent.com/andremillet/odin/master/install.sh | bash
```

This downloads the latest release binary and installs it to `/usr/local/bin`.

## Installation

For manual installation or development:

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

### Update Project
```bash
odin update
```
Adds all changes, commits with a message, and pushes to the remote repository. If run in a Git repository not managed by Odin, it offers to adopt the project by creating a CONFIG.toml and adding it to Odin's project list.

### Upgrade Odin
```bash
odin upgrade
```
Checks for the latest version of Odin on GitHub. If an update is available, displays the release notes and prompts for installation. Downloads and replaces the binary if confirmed.

## Requirements

- Git
- GitHub CLI (`gh`) - automatically installed if missing
- Internet connection for GitHub operations

## Project Structure

- `src/main.rs`: Entry point and CLI definition.
- `src/commands/`: Individual command implementations.
- `Cargo.toml`: Dependencies and project metadata.
- `install.sh`: Installation script.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is open-source. Check the license file for details.