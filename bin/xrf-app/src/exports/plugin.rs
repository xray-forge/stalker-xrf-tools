use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::exports::state::ExportsProjectState;

pub struct ExportsPlugin {}

impl ExportsPlugin {
  pub const NAME: &'static str = crate::tauri_command_registry::exports::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ExportsProjectState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::tauri_command_registry::exports::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::tauri_command_registry::exports::specta_builder()
  }
}
