use crate::equipment::dimensions::get_system_ltx_equipment_sprite_max_dimension;
use crate::{
  read_dds_by_path, save_image_as_dds, PackOptions, INVENTORY_ICON_GRID_SQUARE_BASE,
  SECTION_TYPE_INVENTORY_ICON,
};
use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use image_dds::ImageFormat;
use std::path::PathBuf;
use xray_ltx::Section;

pub fn pack_equipment_icons_by_ltx(options: PackOptions) {
  let (max_width, max_height) = get_system_ltx_equipment_sprite_max_dimension(&options.ltx);

  if max_width > 32 * 1024 || max_height > 32 * 1024 {
    panic!("Trying to create too large resulting dds file over 32k*32k, it is not supported");
  }

  let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(max_width, max_height);

  for (section_name, section) in &options.ltx.sections {
    if let Some(is_type_inventory_icon) = section.get(SECTION_TYPE_INVENTORY_ICON) {
      if is_type_inventory_icon.to_lowercase() == "true" {
        pack_equipment_icon(&options, &mut result, section_name, section);
      }
    }
  }

  save_image_as_dds(&options.output, &result, ImageFormat::BC3RgbaUnorm);
}

pub fn pack_equipment_icon(
  options: &PackOptions,
  into: &mut RgbaImage,
  section_name: &str,
  section: &Section,
) {
  let inv_grid_x: Option<&str> = section.get("inv_grid_x");
  let inv_grid_y: Option<&str> = section.get("inv_grid_y");
  let inv_grid_w: Option<&str> = section.get("inv_grid_width");
  let inv_grid_h: Option<&str> = section.get("inv_grid_height");

  if inv_grid_x.is_none() || inv_grid_y.is_none() || inv_grid_w.is_none() || inv_grid_h.is_none() {
    return;
  }

  let inv_grid_x: u32 = inv_grid_x.unwrap().parse::<u32>().unwrap();
  let inv_grid_y: u32 = inv_grid_y.unwrap().parse::<u32>().unwrap();
  let inv_grid_w: u32 = inv_grid_w.unwrap().parse::<u32>().unwrap();
  let inv_grid_h: u32 = inv_grid_h.unwrap().parse::<u32>().unwrap();

  let x_absolute: u32 = inv_grid_x * INVENTORY_ICON_GRID_SQUARE_BASE;
  let y_absolute: u32 = inv_grid_y * INVENTORY_ICON_GRID_SQUARE_BASE;
  let w_absolute: u32 = inv_grid_w * INVENTORY_ICON_GRID_SQUARE_BASE;
  let h_absolute: u32 = inv_grid_h * INVENTORY_ICON_GRID_SQUARE_BASE;

  if options.is_verbose {
    println!(
      "Packing icon: '{section_name}' x:{inv_grid_x}({x_absolute}), \
     y:{inv_grid_y}({y_absolute}), w:{inv_grid_w}({w_absolute}), h:{inv_grid_h}({h_absolute})"
    );
  }

  let icon_dds_path: &PathBuf = &options.source.join(format!("{section_name}.dds"));
  let icon_dds: RgbaImage = read_dds_by_path(icon_dds_path).unwrap_or_else(|_| {
    panic!(
      "Expected icon DDS to exist for assembling at path {:?}",
      icon_dds_path
    )
  });

  into.copy_from(&icon_dds, x_absolute, y_absolute).unwrap();
}
