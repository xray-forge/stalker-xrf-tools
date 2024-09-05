use tauri::plugin::TauriPlugin;
use tauri::Runtime;

pub struct ConfigsEditorPlugin {}

impl ConfigsEditorPlugin {
  pub const NAME: &'static str = "configs-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .invoke_handler(tauri::generate_handler![
        crate::configs_editor::commands::check_format_configs_path::check_format_configs_path,
        crate::configs_editor::commands::format_configs_path::format_configs_path,
        crate::configs_editor::commands::verify_configs_path::verify_configs_path,
      ])
      .build()
  }
}
