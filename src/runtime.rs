use deno_core::{
  error::AnyError,
  JsRuntime,
  RuntimeOptions
};
use std::{env, rc::Rc};

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