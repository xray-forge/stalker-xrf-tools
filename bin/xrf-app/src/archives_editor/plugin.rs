use crate::archives_editor::state::ArchivesEditorState;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

pub struct ArchivesEditorPlugin {}

impl ArchivesEditorPlugin {
  pub const NAME: &'static str = "archives-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ArchivesEditorState::new());

        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
        crate::archives_editor::commands::close_archives_project::close_archives_project,
        crate::archives_editor::commands::get_archives_project::get_archives_project,
        crate::archives_editor::commands::has_archives_project::has_archives_project,
        crate::archives_editor::commands::open_archives_project::open_archives_project,
        crate::archives_editor::commands::read_archive_file::read_archive_file,
        crate::archives_editor::commands::unpack_archives_path::unpack_archives_path,
      ])
      .build()
  }
}
