use crate::equipment::config::get_section_inventory_coordinates;
use crate::{
  save_image_as_ui_dds, TextureResult, UnpackEquipmentOptions, INVENTORY_ICON_GRID_SQUARE_BASE,
};
use image::GenericImageView;
use xray_ltx::Section;

pub fn unpack_equipment_icons_by_ltx(options: UnpackEquipmentOptions) -> TextureResult {
  let mut count: u32 = 0;

  for (section_name, section) in &options.ltx.sections {
    if unpack_equipment_icon(&options, section_name, section)? {
      count += 1;
    }
  }

  println!("Unpacked {count} icons");

  Ok(())
}

pub fn unpack_equipment_icon(
  options: &UnpackEquipmentOptions,
  section_name: &str,
  section: &Section,
) -> TextureResult<bool> {
  let (inv_grid_x, inv_grid_y, inv_grid_w, inv_grid_h) =
    match get_section_inventory_coordinates(section) {
      None => {
        println!("Skip for possible section: '{section_name}'");

        return Ok(false);
      }
      Some(it) => it,
    };

  if inv_grid_h == 0 || inv_grid_w == 0 {
    println!("Skip for possible section: '{section_name}' - width or height is zero",);

    return Ok(false);
  }

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

  if x_absolute + w_absolute > options.source.width()
    || y_absolute + h_absolute > options.source.height()
  {
    println!("Skip for possible section: '{section_name}' - icon is out of source file bonds");

    Ok(false)
  } else {
    save_image_as_ui_dds(
      &options.output.join(format!("{section_name}.dds")),
      &options
        .source
        .view(x_absolute, y_absolute, w_absolute, h_absolute)
        .to_image(),
      options.dds_compression_format,
    )?;

    Ok(true)
  }
}
