use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::visuals::state::VisualState;

pub struct VisualsPlugin {}

impl VisualsPlugin {
  pub const NAME: &'static str = crate::tauri_command_registry::visuals::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(VisualState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::tauri_command_registry::visuals::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::tauri_command_registry::visuals::specta_builder()
  }
}
