use crate::icons_editor::state::IconsEditorState;
use crate::icons_editor::stream::get_equipment_sprite_stream_response;
use tauri::http::header::CONTENT_TYPE;
use tauri::http::{Response, StatusCode};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub struct IconsEditorPlugin {}

impl IconsEditorPlugin {
  pub const NAME: &'static str = "icons-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(IconsEditorState::new());

        Ok(())
      })
      .register_uri_scheme_protocol("stream", move |application, request| {
        get_equipment_sprite_stream_response(application, &request).unwrap_or_else(|error| {
          log::warn!("Failed to handle stream protocol request: {:?}", error);

          Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(CONTENT_TYPE, "text/plain")
            .body(error.to_string().as_bytes().to_vec())
            .unwrap()
        })
      })
      .invoke_handler(tauri::generate_handler![
        crate::icons_editor::commands::close_equipment_sprite::close_equipment_sprite,
        crate::icons_editor::commands::get_equipment_sprite::get_equipment_sprite,
        crate::icons_editor::commands::open_equipment_sprite::open_equipment_sprite,
        crate::icons_editor::commands::reopen_equipment_sprite::reopen_equipment_sprite,
        crate::icons_editor::commands::pack_equipment::pack_equipment,
      ])
      .build()
  }
}
