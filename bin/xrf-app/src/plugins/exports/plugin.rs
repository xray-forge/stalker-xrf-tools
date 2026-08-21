use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::plugins::exports::state::ExportsProjectState;

pub struct ExportsPlugin {}

impl ExportsPlugin {
  pub const NAME: &'static str = crate::registry::exports::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ExportsProjectState::new());

        Ok(())
      })
      .invoke_handler(crate::app::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::registry::exports::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::registry::exports::specta_builder()
  }
}
