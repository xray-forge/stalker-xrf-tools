use xrf_ltx::Ltx;
use xrf_output::OutputOptions;
use xrf_texture::{ImageFormat, PackEquipmentOptions, PackEquipmentProcessor, PackEquipmentResult};

use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn pack_equipment(
  source_path: &str,
  output_path: &str,
  system_ltx_path: &str,
) -> TauriResult<PackEquipmentResult> {
  let options = PackEquipmentOptions {
    ltx: Ltx::read_from_file_full(system_ltx_path).map_err(|error| error.to_string())?,
    source: source_path.into(),
    output: OutputOptions::default(),
    output_path: output_path.into(),
    gamedata: None,
    dds_compression_format: ImageFormat::BC3RgbaUnorm,
    is_strict: false,
  };

  log::info!("Packing equipment dds: {source_path} -> {output_path}, {system_ltx_path}");

  let result: PackEquipmentResult = PackEquipmentProcessor::pack_sprites(options).map_err(error_to_string)?;

  Ok(result)
}
