use crate::equipment::dimensions::get_system_ltx_equipment_sprite_max_dimension;
use crate::{
  read_dds_by_path, save_image_as_dds, PackEquipmentOptions, INVENTORY_ICON_GRID_SQUARE_BASE,
  SECTION_TYPE_INVENTORY_ICON,
};
use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use image_dds::ImageFormat;
use path_absolutize::*;
use std::io;
use std::path::PathBuf;
use xray_ltx::Section;

pub fn pack_equipment_icons_by_ltx(options: PackEquipmentOptions) {
  let (max_width, max_height) = get_system_ltx_equipment_sprite_max_dimension(&options.ltx);

  if max_width > 32 * 1024 || max_height > 32 * 1024 {
    panic!("Trying to create too large resulting dds file over 32k*32k, it is not supported");
  }

  let mut count: u32 = 0;
  let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(max_width, max_height);

  for (section_name, section) in &options.ltx.sections {
    if let Some(is_type_inventory_icon) = section.get(SECTION_TYPE_INVENTORY_ICON) {
      if is_type_inventory_icon.to_lowercase() == "true" {
        let is_packed: bool = pack_equipment_icon(&options, &mut result, section_name, section);

        if is_packed {
          count += 1;
        }
      }
    }
  }

  save_image_as_dds(&options.output, &result, ImageFormat::BC3RgbaUnorm);

  println!("Packed {count} icons")
}

pub fn pack_equipment_icon(
  options: &PackEquipmentOptions,
  into: &mut RgbaImage,
  section_name: &str,
  section: &Section,
) -> bool {
  let inv_grid_custom: Option<&str> = section.get("$inventory_icon_path");

  let inv_grid_x: Option<&str> = section.get("inv_grid_x");
  let inv_grid_y: Option<&str> = section.get("inv_grid_y");
  let inv_grid_w: Option<&str> = section.get("inv_grid_width");
  let inv_grid_h: Option<&str> = section.get("inv_grid_height");

  if inv_grid_x.is_none() || inv_grid_y.is_none() || inv_grid_w.is_none() || inv_grid_h.is_none() {
    return false;
  }

  let inv_grid_x: u32 = inv_grid_x.unwrap().parse::<u32>().unwrap();
  let inv_grid_y: u32 = inv_grid_y.unwrap().parse::<u32>().unwrap();
  let inv_grid_w: u32 = inv_grid_w.unwrap().parse::<u32>().unwrap();
  let inv_grid_h: u32 = inv_grid_h.unwrap().parse::<u32>().unwrap();

  let x_absolute: u32 = inv_grid_x * INVENTORY_ICON_GRID_SQUARE_BASE;
  let y_absolute: u32 = inv_grid_y * INVENTORY_ICON_GRID_SQUARE_BASE;
  let w_absolute: u32 = inv_grid_w * INVENTORY_ICON_GRID_SQUARE_BASE;
  let h_absolute: u32 = inv_grid_h * INVENTORY_ICON_GRID_SQUARE_BASE;

  let icon_dds_path: PathBuf =
    get_equipment_icon_source_path(options, section_name, inv_grid_custom);

  if options.is_verbose {
    println!(
      "Packing icon: {:?} - '{section_name}' x:{inv_grid_x}({x_absolute}), \
     y:{inv_grid_y}({y_absolute}), w:{inv_grid_w}({w_absolute}), h:{inv_grid_h}({h_absolute})",
      icon_dds_path
    );
  }

  let icon_dds: io::Result<RgbaImage> = read_dds_by_path(&icon_dds_path);

  match icon_dds {
    Ok(icon_dds) => {
      into.copy_from(&icon_dds, x_absolute, y_absolute).unwrap();

      true
    }
    Err(error) => {
      if options.is_strict {
        panic!(
          "Expected icon DDS to exist for assembling at path {:?}",
          icon_dds_path
        );
      } else {
        println!("Skip icon '{section_name}', reason: {:?}", error);
      }

      false
    }
  }
}

pub fn get_equipment_icon_source_path(
  options: &PackEquipmentOptions,
  name: &str,
  custom_path: Option<&str>,
) -> PathBuf {
  match custom_path {
    None => options.source.join(format!("{name}.dds")),
    Some(custom_path) => {
      // Handle custom gamedata source.
      if let Some(gamedata) = &options.gamedata {
        if custom_path.starts_with('~') {
          PathBuf::from(
            gamedata
              .join(custom_path.strip_prefix("~\\").unwrap())
              .absolutize()
              .unwrap()
              .to_str()
              .unwrap(),
          )
        } else {
          PathBuf::from(
            gamedata
              .join("textures")
              .join(custom_path)
              .absolutize()
              .unwrap()
              .to_str()
              .unwrap(),
          )
        }
        // Handle ~ path for xrf / system.ltx
      } else if custom_path.starts_with('~') {
        PathBuf::from(
          options
            .ltx
            .directory
            .as_ref()
            .unwrap()
            .join("..")
            .join("..")
            .join("resources")
            .join(custom_path.strip_prefix("~\\").unwrap())
            .absolutize()
            .unwrap()
            .to_str()
            .unwrap(),
        )
        // Handle relative path for xrf / system.ltx extensions
      } else if custom_path.starts_with('#') {
        PathBuf::from(
          options
            .ltx
            .directory
            .as_ref()
            .unwrap()
            .join("..")
            .join("extensions")
            .join(custom_path.strip_prefix("#\\").unwrap())
            .absolutize()
            .unwrap()
            .to_str()
            .unwrap(),
        )
        // Handle relative path for xrf / system.ltx
      } else {
        PathBuf::from(
          options
            .ltx
            .directory
            .as_ref()
            .unwrap()
            .join("..")
            .join("..")
            .join("resources")
            .join("textures")
            .join(custom_path)
            .absolutize()
            .unwrap()
            .to_str()
            .unwrap(),
        )
      }
    }
  }
}
