#[path = "src/tauri_command_registry/build.rs"]
mod tauri_command_registry;

use tauri_build::{Attributes, WindowsAttributes};

fn main() {
  tauri_build::try_build(tauri_command_registry::configure(
    Attributes::new()
      .codegen(tauri_build::CodegenContext::new())
      .windows_attributes(WindowsAttributes::new()),
  ))
  .expect("failed to run tauri-build")
}
