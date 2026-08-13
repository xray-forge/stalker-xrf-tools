use tauri::Runtime;
use tauri::plugin::TauriPlugin;

pub struct ConfigsEditorPlugin {}

impl ConfigsEditorPlugin {
  pub const NAME: &'static str = "configs-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::configs_editor::commands::check_format_configs_path::check_format_configs_path,
          crate::configs_editor::commands::format_configs_path::format_configs_path,
          crate::configs_editor::commands::verify_configs_path::verify_configs_path,
        ],
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    tauri_specta::Builder::new()
      .plugin_name(Self::NAME)
      .error_handling(tauri_specta::ErrorHandlingMode::Throw)
      .commands(tauri_specta::collect_commands![
        crate::configs_editor::commands::check_format_configs_path::check_format_configs_path,
        crate::configs_editor::commands::format_configs_path::format_configs_path,
        crate::configs_editor::commands::verify_configs_path::verify_configs_path,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
