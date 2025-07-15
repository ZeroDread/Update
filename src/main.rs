use anyhow::Result;
use colored::*;
use console::Term;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

mod commands;
mod config;
mod ui;

use commands::*;
use config::*;

#[tokio::main]
async fn main() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;
    
    // Display beautiful header
    display_header();
    
    // Load configuration
    let _config = Config::load();
    let commands = get_all_commands();
    
    // Create interactive menu
    let theme = ColorfulTheme::default();
    let selections = MultiSelect::with_theme(&theme)
        .with_prompt("Select commands to execute (Space to select, Enter to confirm, Ctrl+C to quit)")
        .items(&commands.iter().map(|c| format!("{} {}", c.icon, c.name)).collect::<Vec<_>>())
        .interact()?;
    
    if selections.is_empty() {
        println!("{}", "No commands selected. Executing all commands...".yellow());
        execute_all_commands(&commands).await?;
    } else {
        let selected_commands: Vec<&UpdateCommand> = selections.iter().map(|&i| &commands[i]).collect();
        execute_selected_commands(&selected_commands).await?;
    }
    
    // Return to home directory
    std::env::set_current_dir(std::env::var("HOME")?)?;
    
    println!("\n{}", "🎉 All tasks completed successfully!".green().bold());
    println!("{} {}", "Currently in:".white(), std::env::current_dir()?.display().to_string().yellow());
    
    Ok(())
}

fn display_header() {
    let title = r#"
    +----------------------------------------------------------+
    |                 Nudge - System Manager                   |
    |                     Built with Rust                      |
    +----------------------------------------------------------+
    "#;
    
    println!("{}", title.cyan().bold());
    println!("{}", "Select tasks to execute:".white());
    println!();
}

async fn execute_all_commands(commands: &[UpdateCommand]) -> Result<()> {
    println!("{}", "\n🔄 Executing all commands...".blue().bold());
    
    for (i, command) in commands.iter().enumerate() {
        execute_command(command, i + 1, commands.len()).await?;
    }
    
    Ok(())
}

async fn execute_selected_commands(commands: &[&UpdateCommand]) -> Result<()> {
    println!("{}", "\n🔄 Executing selected commands...".blue().bold());
    
    for (i, command) in commands.iter().enumerate() {
        execute_command(command, i + 1, commands.len()).await?;
    }
    
    Ok(())
}

async fn execute_command(command: &UpdateCommand, current: usize, total: usize) -> Result<()> {
    let progress_msg = format!("[{}/{}] {} {}", current, total, command.icon, command.name);
    println!("{}", progress_msg.cyan().bold());
    
    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message(format!("Executing: {}", command.name));
    pb.enable_steady_tick(Duration::from_millis(100));
    
    let result = match command.command_type {
        CommandType::Shell => execute_shell_command(&command.command).await,
        CommandType::Custom => execute_custom_command(&command.command).await,
    };
    
    pb.finish_with_message(format!("✅ {}", command.name));
    
    match result {
        Ok(_) => println!("{}", format!("✅ {} completed successfully", command.name).green()),
        Err(e) => {
            println!("{}", format!("❌ {} failed: {}", command.name, e).red());
            return Err(e);
        }
    }
    
    // Minimal delay for terminal output
    sleep(Duration::from_millis(50)).await;
    
    Ok(())
}

async fn execute_shell_command(command: &str) -> Result<()> {
    let output = Command::new("zsh")
        .arg("-c")
        .arg(command)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Command failed: {}", stderr));
    }
    
    Ok(())
}

async fn execute_custom_command(command: &str) -> Result<()> {
    match command {
        "handle_site_repo" => {
            // Change to Developer/Git directory
            let dev_git_path = format!("{}/Developer/Git", std::env::var("HOME")?);
            std::env::set_current_dir(&dev_git_path)?;
            
            // Remove Site directory if it exists
            if std::path::Path::new("Site").exists() {
                println!("{}", "🗑️  Removing existing Site directory...".yellow());
                std::fs::remove_dir_all("Site")?;
            }
            
            // Clone repository
            println!("{}", "📥 Cloning Site repository...".blue());
            let output = Command::new("gh")
                .args(&["repo", "clone", "ZeroDread/Site"])
                .output()?;
            
            if !output.status.success() {
                return Err(anyhow::anyhow!("Failed to clone repository"));
            }
            
            // Change to Site directory and run bun install
            std::env::set_current_dir("Site")?;
            println!("{}", "📦 Installing dependencies with bun...".blue());
            let output = Command::new("bun")
                .arg("install")
                .output()?;
            
            if !output.status.success() {
                return Err(anyhow::anyhow!("Failed to install dependencies"));
            }
            
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Unknown custom command: {}", command)),
    }
}
