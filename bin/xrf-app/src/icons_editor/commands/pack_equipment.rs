use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use xray_ltx::Ltx;
use xray_texture::{
  ImageFormat, PackEquipmentOptions, PackEquipmentProcessor, PackEquipmentResult,
};

#[tauri::command]
pub async fn pack_equipment(
  source_path: &str,
  output_path: &str,
  system_ltx_path: &str,
) -> TauriResult<Value> {
  let options = PackEquipmentOptions {
    ltx: Ltx::read_from_file_full(system_ltx_path).map_err(|error| error.to_string())?,
    source: source_path.into(),
    output: output_path.into(),
    gamedata: None,
    dds_compression_format: ImageFormat::BC3RgbaUnorm,
    is_verbose: false,
    is_strict: false,
  };

  log::info!("Packing equipment dds: {source_path} -> {output_path}, {system_ltx_path}");

  let result: PackEquipmentResult =
    PackEquipmentProcessor::pack_sprites(options).map_err(error_to_string)?;

  Ok(json!(result))
}
