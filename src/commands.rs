use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    Shell,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommand {
    pub name: String,
    pub command: String,
    pub command_type: CommandType,
    pub icon: String,
    pub description: String,
    pub category: String,
}

impl UpdateCommand {
    pub fn new(name: &str, command: &str, command_type: CommandType, icon: &str, description: &str, category: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            command_type,
            icon: icon.to_string(),
            description: description.to_string(),
            category: category.to_string(),
        }
    }
}

pub fn get_all_commands() -> Vec<UpdateCommand> {
    vec![
        UpdateCommand::new(
            "Update Homebrew",
            "brew update && brew upgrade",
            CommandType::Shell,
            "",
            "Update and upgrade all Homebrew packages",
            "Package Managers"
        ),
        UpdateCommand::new(
            "Update npm packages",
            "npm update && npm fund",
            CommandType::Shell,
            "",
            "Update npm packages and show funding information",
            "Package Managers"
        ),
        UpdateCommand::new(
            "Update bun",
            "bun update && bun upgrade",
            CommandType::Shell,
            "",
            "Update bun itself and upgrade to latest version",
            "Package Managers"
        ),
        UpdateCommand::new(
            "Update Ruby gems",
            "gem update",
            CommandType::Shell,
            "",
            "Update all installed Ruby gems",
            "Package Managers"
        ),
        UpdateCommand::new(
            "Update Rust/Cargo packages",
            "cargo install-update --all",
            CommandType::Shell,
            "",
            "Update all Rust packages installed via Cargo",
            "Package Managers"
        ),
        UpdateCommand::new(
            "Sync and upgrade Doom Emacs",
            "~/.config/emacs/bin/doom sync && ~/.config/emacs/bin/doom upgrade && ~/.config/emacs/bin/doom doctor",
            CommandType::Shell,
            "",
            "Sync, upgrade, and run doctor for Doom Emacs",
            "Development Tools"
        ),
        UpdateCommand::new(
            "Set Solana config to mainnet-beta",
            "solana config set --url mainnet-beta",
            CommandType::Shell,
            "",
            "Configure Solana CLI to use mainnet-beta",
            "Blockchain"
        ),
        UpdateCommand::new(
            "Check Solana address",
            "solana address",
            CommandType::Shell,
            "",
            "Display the current Solana wallet address",
            "Blockchain"
        ),
        UpdateCommand::new(
            "Set Solana config to devnet",
            "solana config set --url devnet",
            CommandType::Shell,
            "",
            "Configure Solana CLI to use devnet",
            "Blockchain"
        ),
        UpdateCommand::new(
            "Request Solana airdrop",
            "solana airdrop 5",
            CommandType::Shell,
            "",
            "Request 5 SOL airdrop on devnet",
            "Blockchain"
        ),
        UpdateCommand::new(
            "Check Solana balance",
            "solana balance",
            CommandType::Shell,
            "",
            "Check current SOL balance",
            "Blockchain"
        ),
        UpdateCommand::new(
            "Handle Site repository",
            "handle_site_repo",
            CommandType::Custom,
            "",
            "Clone/update Site repository and install dependencies",
            "Development"
        ),
        UpdateCommand::new(
            "Install software updates",
            "sudo softwareupdate --install --all",
            CommandType::Shell,
            "",
            "Install all available macOS software updates",
            "System"
        ),
        UpdateCommand::new(
            "Run mac-cleanup",
            "sudo mac-cleanup --force",
            CommandType::Shell,
            "",
            "Clean up macOS system files and caches",
            "System"
        ),
    ]
}

// Function to add new commands dynamically
pub fn add_command(commands: &mut Vec<UpdateCommand>, command: UpdateCommand) {
    commands.push(command);
}

// Function to remove a command by name
pub fn remove_command(commands: &mut Vec<UpdateCommand>, name: &str) {
    commands.retain(|cmd| cmd.name != name);
}

// Function to get commands by category
pub fn get_commands_by_category<'a>(commands: &'a [UpdateCommand], category: &str) -> Vec<&'a UpdateCommand> {
    commands.iter().filter(|cmd| cmd.category == category).collect()
}

// Function to get all categories
pub fn get_all_categories(commands: &[UpdateCommand]) -> Vec<String> {
    let mut categories: Vec<String> = commands.iter().map(|cmd| cmd.category.clone()).collect();
    categories.sort();
    categories.dedup();
    categories
}
