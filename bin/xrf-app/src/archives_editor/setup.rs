use crate::archives_editor::state::ArchivesEditorState;
use std::sync::{Arc, Mutex};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

pub fn init_archives_editor<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("archives_editor")
    .setup(|application| {
      application.manage(ArchivesEditorState {
        project: Arc::new(Mutex::new(None)),
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      crate::archives_editor::commands::close_archives_project,
      crate::archives_editor::commands::get_archives_project,
      crate::archives_editor::commands::has_archives_project,
      crate::archives_editor::commands::open_archives_project,
      crate::archives_editor::commands::read_archive_file,
      crate::archives_editor::commands::unpack_archives_path,
    ])
    .build()
}
