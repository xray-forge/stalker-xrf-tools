use tauri::plugin::TauriPlugin;
use tauri::Runtime;

pub fn init_configs_editor<R: Runtime>() -> TauriPlugin<R> {
  tauri::plugin::Builder::new("configs_editor")
    .invoke_handler(tauri::generate_handler![
      crate::configs_editor::commands::check_format_configs_path,
      crate::configs_editor::commands::format_configs_path,
      crate::configs_editor::commands::verify_configs_path,
    ])
    .build()
}
