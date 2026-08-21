use tauri::Runtime;
use tauri::plugin::TauriPlugin;

pub struct ConfigsPlugin {}

impl ConfigsPlugin {
  pub const NAME: &'static str = crate::registry::configs::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .invoke_handler(crate::app::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::registry::configs::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::registry::configs::specta_builder()
  }
}
