mod runtime;
mod init;
mod tasks;
mod fmt;
mod wasm;

use colored::Colorize;
use std::thread::{self, JoinHandle};
use tokio::runtime::Builder;
use runtime::JsExecutor;
use fmt::Formatter;
use wasm::WasmExecutor; // Import WasmExecutor
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zen")]
#[command(about = "A JavaScript runtime", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        files: Vec<String>,
    },
    Init,
    Task {
        name: String,
    },
    Fmt {
        files: Vec<String>,
    },
    Wasm {
        file: String,
    },
}

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

fn execute_wasm(file_path: String) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let executor = WasmExecutor::new();
        executor.run(&file_path)
    })
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { files } => {
            if files.is_empty() {
                eprintln!("{}", "No files specified to run.".red());
                return;
            }

            let handles: Vec<_> = files
                .iter()
                .map(|file| execute_file(file.to_string()))
                .collect();

            for (file, handle) in files.iter().zip(handles) {
                match handle.join() {
                    Ok(result) => {
                        if let Err(error) = result {
                            eprintln!("Error executing file {}: {}", file.red(), error.to_string().red().bold());
                        } else {
                            println!("Successfully executed file {}", file.green());
                        }
                    }
                    Err(_) => {
                        eprintln!("Thread for file {} panicked", file.red());
                    }
                }
            }
        }
        Commands::Wasm { file } => {
            let handle = execute_wasm(file);
            match handle.join() {
                Ok(result) => {
                    if let Err(error) = result {
                        eprintln!("Error executing WASM file {}: {}", file.red(), error.red().bold());
                    } else {
                        println!("Successfully executed WASM file {}", file.green());
                    }
                }
                Err(_) => {
                    eprintln!("Thread for WASM file {} panicked", file.red());
                }
            }
        }
        Commands::Init => {
            init::initialize_project();
        }
        Commands::Task { name } => {
            tasks::handle_task(&name);
        }
        Commands::Fmt { files } => {
            let formatter = match Formatter::new() {
                Ok(fmt) => fmt,
                Err(e) => {
                    eprintln!("Error initializing formatter: {}", e.to_string().red().bold());
                    return;
                }
            };

            for file in files {
                match formatter.format_js(&file) {
                    Ok(_) => println!("Formatted {}", file),
                    Err(e) => eprintln!("Error formatting file {}: {}", file, e.to_string().red().bold()),
                }
            }
        }
    }
}