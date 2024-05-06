use crate::{
  save_image_as_dds, UnpackOptions, INVENTORY_ICON_GRID_SQUARE_BASE, SECTION_TYPE_INVENTORY_ICON,
};
use image::GenericImageView;
use image_dds::ImageFormat;
use xray_ltx::Section;

pub fn unpack_equipment_icons_by_ltx(options: UnpackOptions) {
  let mut count: u32 = 0;

  for (section_name, section) in &options.ltx.sections {
    if let Some(is_type_inventory_icon) = section.get(SECTION_TYPE_INVENTORY_ICON) {
      if is_type_inventory_icon.to_lowercase() == "true" {
        unpack_equipment_icon(&options, &section_name, &section);
        count += 1;
      }
    }
  }

  println!("Unpacked {count} icons")
}

pub fn unpack_equipment_icon(options: &UnpackOptions, section_name: &str, section: &Section) {
  let inv_grid_x: Option<&str> = section.get("inv_grid_x");
  let inv_grid_y: Option<&str> = section.get("inv_grid_y");
  let inv_grid_w: Option<&str> = section.get("inv_grid_width");
  let inv_grid_h: Option<&str> = section.get("inv_grid_height");

  if inv_grid_x.is_none() || inv_grid_y.is_none() || inv_grid_w.is_none() || inv_grid_h.is_none() {
    println!("Skip for possible section: '{section_name}'");

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
      "Unpacking icon: '{section_name}' x:{inv_grid_x}({x_absolute}), \
     y:{inv_grid_y}({y_absolute}), w:{inv_grid_w}({w_absolute}), h:{inv_grid_h}({h_absolute})"
    );
  }

  save_image_as_dds(
    &options.output.join(format!("{section_name}.dds")),
    &options
      .source
      .view(x_absolute, y_absolute, w_absolute, h_absolute)
      .to_image(),
    ImageFormat::BC3RgbaUnorm,
  );
}
