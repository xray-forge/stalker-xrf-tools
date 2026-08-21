use tauri::Runtime;
use tauri::plugin::TauriPlugin;

/// Desktop integration that belongs to no editor in particular.
pub struct SystemPlugin {}

impl SystemPlugin {
  pub const NAME: &'static str = crate::ipc::registry::system::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .invoke_handler(crate::core::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::ipc::registry::system::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::ipc::registry::system::specta_builder()
  }
}
