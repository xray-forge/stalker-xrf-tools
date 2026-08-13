use tauri::Runtime;
use tauri::plugin::TauriPlugin;

pub struct ConfigsPlugin {}

impl ConfigsPlugin {
  pub const NAME: &'static str = "configs";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::configs::commands::check_directory_format::configs_check_directory_format,
          crate::configs::commands::format_directory::configs_format_directory,
          crate::configs::commands::verify_directory::configs_verify_directory,
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
        crate::configs::commands::check_directory_format::configs_check_directory_format,
        crate::configs::commands::format_directory::configs_format_directory,
        crate::configs::commands::verify_directory::configs_verify_directory,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
