use colored::*;
use std::fmt::Arguments;

pub fn log_error(args: Arguments) {
    // eprintln!("{}", format!("Error: {}", message).red().bold());
    eprintln!("{}", format!("❌ Error: {}", args).red().bold());
}

pub fn log_info(args: Arguments) {
    println!("{}", format!("✅ Info: {}", args).green().bold());
}

pub fn log_notice(args: Arguments) {
    println!("{}", format!("🔔 {}", args).yellow().bold());
}
