use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;
use colored::*;

pub fn initialize_project() {
    let config = r#"{
    "name": "project",
    "version": "1.0.0",
    "tasks": {
        "dev": "zen run src/main.js"
    }
}"#;

    if Path::new("zen.json").exists() {
        eprintln!("{} zen.json already exists. Initialization aborted. {}", "Error:".red().bold(), "");
        return;
    }

    let mut file = match File::create("zen.json") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} Failed to create zen.json: {}{}", "Error:".red().bold(), e, "");
            return;
        }
    };

    if let Err(e) = file.write_all(config.as_bytes()) {
        eprintln!("{} Failed to write to zen.json: {}{}", "Error:".red().bold(), e, "");
    } else {
        println!("{} Project initialized successfully with zen.json.{}", "Success:".green().bold(), "");
    }

    // Create the src directory if it doesn't exist
    if let Err(e) = create_dir_all("src") {
        eprintln!("{} Failed to create src directory: {}{}", "Error:".red().bold(), e, "");
        return;
    }

    // Create src/main.js
    let main_js_content = r#"console.log("Hello Zen!");
"#;

    if let Err(e) = std::fs::write("src/main.js", main_js_content) {
        eprintln!("{} Failed to create src/main.js: {}{}", "Error:".red().bold(), e, "");
    } else {
        println!("{} src/main.js created successfully.", "Success:".green().bold());
    }
}