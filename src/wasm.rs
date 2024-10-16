use wasmtime::{Engine, Module, Instance, Store, Linker};
use std::path::Path;

pub struct WasmExecutor {
    engine: Engine,
}

impl WasmExecutor {
    pub fn new() -> Self {
        let engine = Engine::default();
        WasmExecutor { engine }
    }

    pub fn run(&self, file_path: &str) -> Result<(), String> {
        let store = Store::new(&self.engine);
        let module = Module::from_file(&self.engine, file_path).map_err(|e| e.to_string())?;
        let instance = Instance::new(&store, &module, &[]).map_err(|e| e.to_string())?;

        if let Some(start) = instance.get_func("_start") {
            start.call(&[]).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}