use crate::equipment::config::get_section_inventory_coordinates;
use crate::equipment::dimensions::get_system_ltx_equipment_sprite_max_dimension;
use crate::images::dds_to_image;
use crate::{
  read_dds_by_path, rescale_image_to_bounds, save_image_as_ui_dds, PackEquipmentOptions,
  INVENTORY_ICON_GRID_SQUARE_BASE, SECTION_TYPE_INVENTORY_ICON,
};
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba, RgbaImage};
use path_absolutize::*;
use std::error::Error;
use std::path::{Path, PathBuf};
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

  assert_eq!(
    result.width() % 4,
    0,
    "DirectX compression requires texture width to be multiple of 4"
  );
  assert_eq!(
    result.height() % 4,
    0,
    "DirectX compression requires texture height to be multiple of 4"
  );

  save_image_as_ui_dds(&options.output, &result, options.dds_compression_format);

  println!(
    "Packed {count} icons in {} format",
    options.dds_compression_format
  )
}

pub fn pack_equipment_icon(
  options: &PackEquipmentOptions,
  into: &mut RgbaImage,
  section_name: &str,
  section: &Section,
) -> bool {
  let (inv_grid_x, inv_grid_y, inv_grid_w, inv_grid_h) =
    match get_section_inventory_coordinates(section) {
      None => return false,
      Some(it) => it,
    };

  let x_absolute: u32 = inv_grid_x * INVENTORY_ICON_GRID_SQUARE_BASE;
  let y_absolute: u32 = inv_grid_y * INVENTORY_ICON_GRID_SQUARE_BASE;
  let w_absolute: u32 = inv_grid_w * INVENTORY_ICON_GRID_SQUARE_BASE;
  let h_absolute: u32 = inv_grid_h * INVENTORY_ICON_GRID_SQUARE_BASE;

  let inv_grid_custom: Option<&str> = section.get("$inventory_icon_path");

  let icon_path: PathBuf = get_equipment_icon_source_path(options, section_name, inv_grid_custom);
  let icon = get_equipment_image_from_path(&icon_path, w_absolute, h_absolute);

  match icon {
    Ok(image) => {
      if options.is_verbose {
        println!(
          "Packing icon: {:?} - '{section_name}' x:{inv_grid_x}({x_absolute}), \
   y:{inv_grid_y}({y_absolute}), w:{inv_grid_w}({w_absolute}), h:{inv_grid_h}({h_absolute}), \
    {}x{}",
          icon_path,
          image.width(),
          image.height(),
        );
      }

      into.copy_from(&image, x_absolute, y_absolute).unwrap();

      true
    }
    Err(error) => {
      if options.is_strict {
        panic!(
          "Expected icon to exist for assembling at path {:?} / {section_name}",
          icon_path
        );
      } else {
        println!(
          "Skip icon {:?} / '{section_name}', reason: {:?}",
          icon_path, error
        );
      }

      false
    }
  }
}

pub fn get_equipment_image_from_path(
  path: &Path,
  width: u32,
  height: u32,
) -> Result<DynamicImage, impl Error> {
  if path
    .extension()
    .is_some_and(|extension| extension.eq("png"))
  {
    let icon_png: DynamicImage = ImageReader::open(path)?.decode().unwrap();

    return Ok(rescale_image_to_bounds(icon_png, width, height));
  }

  read_dds_by_path(path)
    .and_then(|dds| dds_to_image(&dds))
    .map(|image| rescale_image_to_bounds(image.into(), width, height))
}

pub fn get_equipment_icon_source_path(
  options: &PackEquipmentOptions,
  name: &str,
  custom_path: Option<&str>,
) -> PathBuf {
  match custom_path {
    None => {
      let png_path: PathBuf = options.source.join(format!("{name}.png"));

      if png_path.exists() {
        png_path
      } else {
        options.source.join(format!("{name}.dds"))
      }
    }
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
