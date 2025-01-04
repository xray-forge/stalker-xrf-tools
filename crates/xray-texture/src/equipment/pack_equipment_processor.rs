use crate::data::inventory_sprite_descriptor::InventorySpriteDescriptor;
use crate::error::texture_processing_error::TextureProcessingError;
use crate::utils::images::dds_to_image;
use crate::{
  read_dds_by_path, rescale_image_to_bounds, save_image_as_ui_dds, PackEquipmentOptions,
  PackEquipmentResult, TextureResult,
};
use image::{DynamicImage, GenericImage, ImageBuffer, ImageReader, Rgba};
use path_absolutize::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct PackEquipmentProcessor {}

impl PackEquipmentProcessor {
  pub fn pack_sprites(options: PackEquipmentOptions) -> TextureResult<PackEquipmentResult> {
    let started_at: Instant = Instant::now();

    let mut count: u32 = 0;
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
      InventorySpriteDescriptor::create_equipment_sprite_base_for_ltx(&options.ltx)?;

    for (section_name, section) in &options.ltx.sections {
      if let Some(sprite_descriptor) =
        InventorySpriteDescriptor::new_optional_from_section(section_name, section)
      {
        if let Some((sprite_path, sprite)) = Self::read_sprite(&options, &sprite_descriptor)? {
          let (x, y, w, h) = sprite_descriptor.get_boundaries();

          if options.is_verbose {
            println!(
              "Packing icon: {:?} - '{}' x:{}({x}), y:{}({y}), w:{}({w}), h:{}({h}), {}x{}",
              sprite_path,
              sprite_descriptor.section,
              sprite_descriptor.x,
              sprite_descriptor.y,
              sprite_descriptor.w,
              sprite_descriptor.h,
              sprite.width(),
              sprite.height(),
            );
          }

          image.copy_from(&sprite, x, y)?;
          count += 1;
        }
      }
    }

    assert_eq!(
      image.width() % 4,
      0,
      "DirectX compression requires texture width to be multiple of 4"
    );
    assert_eq!(
      image.height() % 4,
      0,
      "DirectX compression requires texture height to be multiple of 4"
    );

    save_image_as_ui_dds(&options.output, &image, options.dds_compression_format)?;

    println!(
      "Packed {} icons in {} format",
      count, options.dds_compression_format
    );

    Ok(PackEquipmentResult {
      duration: started_at.elapsed().as_millis(),
      saved_at: options.output.clone(),
      saved_width: image.width(),
      saved_height: image.height(),
      packed_count: count,
    })
  }

  pub fn read_sprite(
    options: &PackEquipmentOptions,
    sprite: &InventorySpriteDescriptor,
  ) -> TextureResult<Option<(PathBuf, DynamicImage)>> {
    let (_, _, w, h) = sprite.get_boundaries();
    let sprite_path: PathBuf = Self::read_sprite_path(options, sprite);

    match Self::read_sprite_from_path(&sprite_path, w, h) {
      Ok(icon) => Ok(Some((sprite_path, icon))),
      Err(error) => {
        if options.is_strict {
          Err(TextureProcessingError::new_texture_error(format!(
            "Expected icon to exist for assembling at path {:?} / {}, error: {:?}",
            sprite_path, sprite.section, error
          )))
        } else {
          println!(
            "Skip icon {:?} / '{}', reason: {:?}",
            sprite_path, sprite.section, error
          );

          Ok(None)
        }
      }
    }
  }

  /// Read rescaled png or dds icon to inject into one large equipment file.
  pub fn read_sprite_from_path(
    path: &Path,
    width: u32,
    height: u32,
  ) -> TextureResult<DynamicImage> {
    if path
      .extension()
      .is_some_and(|extension| extension.eq("png"))
    {
      return Ok(rescale_image_to_bounds(
        ImageReader::open(path)?.decode()?,
        width,
        height,
      ));
    }

    read_dds_by_path(path)
      .and_then(|dds| dds_to_image(&dds))
      .map(|image| rescale_image_to_bounds(image.into(), width, height))
  }

  /// Read equipment icon from custom path defined in ltx config folder.
  pub fn read_sprite_path(
    options: &PackEquipmentOptions,
    descriptor: &InventorySpriteDescriptor,
  ) -> PathBuf {
    match descriptor.custom_icon.as_deref() {
      None => {
        let png_path: PathBuf = options.source.join(format!("{}.png", descriptor.section));

        if png_path.exists() {
          png_path
        } else {
          options.source.join(format!("{}.dds", descriptor.section))
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
}
