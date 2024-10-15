mod runtime;
mod init;
mod tasks;
mod fmt;

use colored::Colorize;
use std::{env, thread::{self, JoinHandle}};
use tokio::runtime::Builder;
use runtime::JsExecutor;
use fmt::Formatter;

fn execute_file(file_path: String) -> JoinHandle<Result<(), deno_core::error::AnyError>> {
    thread::spawn(move || {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let mut executor = JsExecutor::new()?;
        runtime.block_on(executor.run_js(&file_path))
    })
}

fn print_manual(command: &str) {
    match command {
        "run" => {
            println!("{}", "Usage: zen run <file1.js> <file2.js> ...".green());
            println!("Executes the specified JavaScript files.");
        }
        "init" => {
            println!("{}", "Usage: zen init".green());
            println!("Initializes a new project.");
        }
        "task" => {
            println!("{}", "Usage: zen task <task_name>".green());
            println!("Handles the specified task.");
        }
        "fmt" => {
            println!("{}", "Usage: zen fmt <file1.js> <file2.js> ...".green());
            println!("Formats the specified JavaScript files.");
        }
        _ => {
            println!("{}", "Available commands: run, init, fmt, task".green());
            println!("Use zen <command> for more information on a command.");
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", "Usage: zen <command>".yellow().bold());
        return;
    }

    let command: &String = &args[1];
    if args.len() == 2 {
        print_manual(command);
        return;
    }

    match command.as_str() {
        "run" => {
            let handles: Vec<_> = args[2..]
                .iter()
                .map(|file| execute_file(file.to_string()))
                .collect();

            for handle in handles {
                if let Err(error) = handle.join().unwrap() {
                    eprintln!("Error executing file: {}", error.to_string().red().bold());
                }
            }
        }
        "init" => {
            init::initialize_project();
        }
        "task" => {
            tasks::handle_task(&args[2]);
        }
        "fmt" => {
            let formatter = Formatter::new(2, 80, fmt::QuoteStyle::Double, true);
            for file in &args[2..] {
                match formatter.format_js(file) {
                    Ok(_) => println!("Formatted {}", file),
                    Err(e) => eprintln!("Error formatting file {}: {}", file, e.to_string().red().bold()),
                }
            }
        }
        _ => {
            eprintln!("{}", "Invalid command. Commands: Run, Init, Fmt, Task".red().bold());
        }
    }
}