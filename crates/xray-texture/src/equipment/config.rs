use crate::equipment::config_inventory_section_descriptor::ConfigInventorySectionDescriptor;
use xray_ltx::{Ltx, Section};

pub fn get_ltx_inventory_descriptors(config: &Ltx) -> Vec<ConfigInventorySectionDescriptor> {
  let mut inventory_sections: Vec<ConfigInventorySectionDescriptor> = Vec::new();

  for (section_name, section) in &config.sections {
    match get_section_inventory_coordinates(section) {
      None => continue,
      Some((x, y, w, h)) => {
        inventory_sections.push(ConfigInventorySectionDescriptor {
          name: section_name.into(),
          x,
          y,
          w,
          h,
        });
      }
    }
  }

  inventory_sections
}

pub fn get_section_inventory_coordinates(section: &Section) -> Option<(u32, u32, u32, u32)> {
  let inv_grid_x: Option<&str> = section.get("inv_grid_x");
  let inv_grid_y: Option<&str> = section.get("inv_grid_y");
  let inv_grid_w: Option<&str> = section.get("inv_grid_width");
  let inv_grid_h: Option<&str> = section.get("inv_grid_height");

  if inv_grid_x.is_none() || inv_grid_y.is_none() || inv_grid_w.is_none() || inv_grid_h.is_none() {
    return None;
  }

  Some((
    inv_grid_x.unwrap().parse::<u32>().unwrap_or(0),
    inv_grid_y.unwrap().parse::<u32>().unwrap_or(0),
    inv_grid_w.unwrap().parse::<u32>().unwrap_or(0),
    inv_grid_h.unwrap().parse::<u32>().unwrap_or(0),
  ))
}
