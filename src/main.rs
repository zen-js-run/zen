use colored::Colorize;
use deno_core::{error::AnyError, JsRuntime, RuntimeOptions};
use std::{
    env,
    rc::Rc,
    thread::{self, JoinHandle},
};
use tokio::runtime::Builder;

pub struct JsExecutor {
    js_runtime: JsRuntime,
}

impl JsExecutor {
    pub fn new() -> Result<Self, AnyError> {
        let js_runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            ..Default::default()
        });
        Ok(JsExecutor { js_runtime })
    }

    pub async fn run_js(&mut self, file_path: &str) -> Result<(), AnyError> {
        let main_module = deno_core::resolve_path(file_path, &env::current_dir()?)?;
        let mod_id = self.js_runtime.load_main_es_module(&main_module).await?;
        self.js_runtime.mod_evaluate(mod_id).await?;
        self.js_runtime.run_event_loop(Default::default()).await
    }
}

fn execute_file(file_path: String) -> JoinHandle<Result<(), AnyError>> {
    // Now takes file_path by value (String)
    thread::spawn(move || {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let mut executor = JsExecutor::new()?;
        runtime.block_on(executor.run_js(&file_path)) // Pass &file_path
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", "Usage: zen <file>...".yellow().bold());
        return;
    }

    let handles: Vec<_> = args[1..]
        .iter()
        .map(|file| execute_file(file.to_string())) // Pass file.to_string()
        .collect();

    for handle in handles {
        if let Err(error) = handle.join().unwrap() {
            eprintln!("error: {}", error);
        }
    }
}