use serde_json::{json, Value};
use xray_icon::{
  pack_equipment_icons_by_ltx, ImageFormat, PackEquipmentOptions, PackEquipmentResult,
};
use xray_ltx::Ltx;

#[tauri::command]
pub async fn pack_equipment(
  source_path: &str,
  output_path: &str,
  system_ltx_path: &str,
) -> Result<Value, String> {
  let options = PackEquipmentOptions {
    ltx: Ltx::load_from_file_full(system_ltx_path).map_err(|error| error.to_string())?,
    source: source_path.into(),
    output: output_path.into(),
    gamedata: None,
    dds_compression_format: ImageFormat::BC3RgbaUnorm,
    is_verbose: false,
    is_strict: false,
  };

  log::info!("Packing equipment dds: {source_path} -> {output_path}, {system_ltx_path}");

  let result: PackEquipmentResult = pack_equipment_icons_by_ltx(options);

  Ok(json!(result))
}
