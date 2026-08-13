use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::archives::state::ArchiveProjectState;

pub struct ArchivesPlugin {}

impl ArchivesPlugin {
  pub const NAME: &'static str = "archives";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ArchiveProjectState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::archives::commands::close_project::archives_close_project,
          crate::archives::commands::extract_file::archives_extract_file,
          crate::archives::commands::extract_directory::archives_extract_directory,
          crate::archives::commands::get_project::archives_get_project,
          crate::archives::commands::has_project::archives_has_project,
          crate::archives::commands::open_project::archives_open_project,
          crate::archives::commands::read_audio::archives_read_audio,
          crate::archives::commands::read_file::archives_read_file,
          crate::archives::commands::read_image::archives_read_image,
          crate::archives::commands::unpack_directory::archives_unpack_directory,
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
        crate::archives::commands::close_project::archives_close_project,
        crate::archives::commands::extract_file::archives_extract_file,
        crate::archives::commands::extract_directory::archives_extract_directory,
        crate::archives::commands::get_project::archives_get_project,
        crate::archives::commands::has_project::archives_has_project,
        crate::archives::commands::open_project::archives_open_project,
        crate::archives::commands::read_audio::archives_read_audio,
        crate::archives::commands::read_file::archives_read_file,
        crate::archives::commands::read_image::archives_read_image,
        crate::archives::commands::unpack_directory::archives_unpack_directory,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
