use std::process::Command;
use std::fs;
use serde::Deserialize;
use colored::Colorize;

#[derive(Deserialize)]
struct ZenConfig {
    tasks: std::collections::HashMap<String, String>,
}

pub fn handle_task(task_name: &str) {
    let config = match fs::read_to_string("zen.json") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("{}", "Could not read zen.json. Make sure it exists.".red().bold());
            return;
        }
    };

    let zen_config: ZenConfig = match serde_json::from_str(&config) {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!("Invalid zen.json format.");
            return;
        }
    };

    if let Some(command) = zen_config.tasks.get(task_name) {
        match Command::new("sh").arg("-c").arg(command).status() {
            Ok(status) if status.success() => println!("Task '{}' completed.", task_name),
            Ok(_) | Err(_) => eprintln!("Task '{}' failed.", task_name),
        }
    } else {
        eprintln!("Task '{}' not found.", task_name);
    }
}