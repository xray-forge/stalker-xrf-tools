use crate::{
  read_dds, save_image_as_dds, INVENTORY_ICON_GRID_SQUARE_BASE, SECTION_TYPE_INVENTORY_ICON,
};
use image::{GenericImage, RgbaImage};
use image_dds::ImageFormat;
use std::cmp::max;
use std::path::Path;
use xray_ltx::{Ltx, Section};

pub fn pack_ltx(ltx: &Ltx, source: &Path, output: &Path) {
  let (max_width, max_height) = get_system_ltx_max_dimension(&ltx);

  if max_width > 32 * 1024 || max_height > 32 * 1024 {
    panic!("Trying to create too large resulting dds file over 32k*32k, it is not supported");
  }

  let mut result = RgbaImage::new(max_width, max_height);

  for (section_name, section) in &ltx.sections {
    if let Some(is_type_inventory_icon) = section.get(SECTION_TYPE_INVENTORY_ICON) {
      if is_type_inventory_icon.to_lowercase() == "true" {
        pack_icon(source, &mut result, &section_name, &section);
      }
    }
  }

  save_image_as_dds(output, &result, ImageFormat::BC3RgbaUnorm);
}

pub fn pack_icon(folder: &Path, target: &mut RgbaImage, section_name: &str, section: &Section) {
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

  target
    .copy_from(
      &read_dds(&folder.join(format!("{section_name}.dds"))),
      inv_grid_x.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
      inv_grid_y.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
    )
    .unwrap();
}

pub fn get_system_ltx_max_dimension(ltx: &Ltx) -> (u32, u32) {
  let mut max_width: u32 = 0;
  let mut max_height: u32 = 0;

  for (_, section) in &ltx.sections {
    if let Some(is_type_inventory_icon) = section.get(SECTION_TYPE_INVENTORY_ICON) {
      if is_type_inventory_icon.to_lowercase() == "true" {
        let (width, height) = get_section_max_dimension(&section);

        max_width = max(width, max_width);
        max_height = max(height, max_height);
      }
    }
  }

  (max_width, max_height)
}

pub fn get_section_max_dimension(section: &Section) -> (u32, u32) {
  let inv_grid_x: Option<&str> = section.get("inv_grid_x");
  let inv_grid_y: Option<&str> = section.get("inv_grid_y");
  let inv_grid_width: Option<&str> = section.get("inv_grid_width");
  let inv_grid_height: Option<&str> = section.get("inv_grid_height");

  if inv_grid_x.is_none()
    || inv_grid_y.is_none()
    || inv_grid_width.is_none()
    || inv_grid_height.is_none()
  {
    (0, 0)
  } else {
    (
      inv_grid_x.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE
        + inv_grid_width.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
      inv_grid_y.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE
        + inv_grid_height.unwrap().parse::<u32>().unwrap() * INVENTORY_ICON_GRID_SQUARE_BASE,
    )
  }
}
