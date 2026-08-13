use tauri::http::header::CONTENT_TYPE;
use tauri::http::{Response, StatusCode};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::equipment_icons::state::EquipmentSpriteState;
use crate::equipment_icons::stream::get_sprite_stream_response;

pub struct EquipmentIconsPlugin {}

impl EquipmentIconsPlugin {
  pub const NAME: &'static str = "equipment-icons";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(EquipmentSpriteState::new());

        Ok(())
      })
      .register_uri_scheme_protocol("stream", move |context, request| {
        get_sprite_stream_response(context, &request).unwrap_or_else(|error| {
          log::warn!("Failed to handle stream protocol request: {}", error);

          Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(CONTENT_TYPE, "text/plain")
            .body(error.to_string().as_bytes().to_vec())
            .unwrap()
        })
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::equipment_icons::commands::close_sprite::equipment_icons_close_sprite,
          crate::equipment_icons::commands::get_sprite::equipment_icons_get_sprite,
          crate::equipment_icons::commands::open_sprite::equipment_icons_open_sprite,
          crate::equipment_icons::commands::reopen_sprite::equipment_icons_reopen_sprite,
          crate::equipment_icons::commands::pack_sprite::equipment_icons_pack_sprite,
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
        crate::equipment_icons::commands::close_sprite::equipment_icons_close_sprite,
        crate::equipment_icons::commands::get_sprite::equipment_icons_get_sprite,
        crate::equipment_icons::commands::open_sprite::equipment_icons_open_sprite,
        crate::equipment_icons::commands::reopen_sprite::equipment_icons_reopen_sprite,
        crate::equipment_icons::commands::pack_sprite::equipment_icons_pack_sprite,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
