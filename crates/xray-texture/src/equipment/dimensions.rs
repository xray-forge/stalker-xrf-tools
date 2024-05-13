use crate::{INVENTORY_ICON_GRID_SQUARE_BASE, SECTION_TYPE_INVENTORY_ICON};
use std::cmp::max;
use xray_ltx::{Ltx, Section};

pub fn get_system_ltx_equipment_sprite_max_dimension(ltx: &Ltx) -> (u32, u32) {
  let mut max_width: u32 = 0;
  let mut max_height: u32 = 0;

  for (_, section) in &ltx.sections {
    if let Some(is_type_inventory_icon) = section.get(SECTION_TYPE_INVENTORY_ICON) {
      if is_type_inventory_icon.to_lowercase() == "true" {
        let (width, height) = get_section_sprite_max_dimension(section);

        max_width = max(width, max_width);
        max_height = max(height, max_height);
      }
    }
  }

  // Make sure resulting sprites are multiples of 4 for width and height
  max_width = max_width + (4 - max_width % 4);
  max_height = max_height + (4 - max_height % 4);

  (max_width, max_height)
}

pub fn get_section_sprite_max_dimension(section: &Section) -> (u32, u32) {
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
