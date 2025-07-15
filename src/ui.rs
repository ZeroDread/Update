use colored::*;
use std::io::{self, Write};

pub fn print_banner() {
    let banner = r#"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║                    System Update Manager                     ║
    ║                                                              ║
    ║                       Built with Rust                        ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
    "#;
    println!("{}", banner.cyan().bold());
}

pub fn print_separator() {
    println!("{}", "─".repeat(60).bright_black());
}

pub fn print_success(message: &str) {
    println!("{} {}", "✅".green(), message.green());
}

pub fn print_error(message: &str) {
    println!("{} {}", "❌".red(), message.red());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message.blue());
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️".yellow(), message.yellow());
}

pub fn pause_for_input() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_progress(current: usize, total: usize, message: &str) {
    let progress = (current as f64 / total as f64) * 100.0;
    let bar_length = 30;
    let filled_length = (progress * bar_length as f64 / 100.0) as usize;
    
    let bar = "█".repeat(filled_length) + &"░".repeat(bar_length - filled_length);
    
    println!(
        "{} [{}/{}] [{}] {:.1}% - {}",
        "🔄".cyan(),
        current,
        total,
        bar.bright_green(),
        progress,
        message.white()
    );
}
