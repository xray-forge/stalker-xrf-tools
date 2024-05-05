use crate::{save_image_as_dds, INVENTORY_ICON_GRID_SQUARE_BASE, SECTION_TYPE_INVENTORY_ICON};
use image::{GenericImageView, RgbaImage};
use image_dds::ImageFormat;
use std::path::{Path, PathBuf};
use xray_ltx::{Ltx, Section};

pub fn unpack_ltx(output: &Path, ltx: &Ltx, source: &RgbaImage) {
  for (section_name, section) in &ltx.sections {
    if let Some(is_type_inventory_icon) = section.get(SECTION_TYPE_INVENTORY_ICON) {
      if is_type_inventory_icon.to_lowercase() == "true" {
        unpack_inventory_icon(output, &section_name, &section, &source);
      }
    }
  }
}

pub fn unpack_inventory_icon(
  output: &Path,
  saved_file_name: &str,
  section: &Section,
  source: &RgbaImage,
) {
  let inv_grid_x: Option<&str> = section.get("inv_grid_x");
  let inv_grid_y: Option<&str> = section.get("inv_grid_y");
  let inv_grid_width: Option<&str> = section.get("inv_grid_width");
  let inv_grid_height: Option<&str> = section.get("inv_grid_height");

  if inv_grid_x.is_none()
    || inv_grid_y.is_none()
    || inv_grid_width.is_none()
    || inv_grid_height.is_none()
  {
    return;
  }

  save_image_as_dds(
    &PathBuf::from(output.join(format!("{saved_file_name}.dds"))),
    &source
      .view(
        inv_grid_x.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
        inv_grid_y.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
        inv_grid_width.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
        inv_grid_height.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
      )
      .to_image(),
    ImageFormat::BC3RgbaUnorm,
  );
}
