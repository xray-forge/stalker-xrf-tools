use crate::data::inventory_sprite_descriptor::InventorySpriteDescriptor;
use crate::utils::images::dds_to_image;
use crate::{PackEquipmentOptions, PackEquipmentResult, read_dds_by_path, save_image_as_ui_dds};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageReader, Rgba, RgbaImage};
use path_absolutize::*;
use std::path::{Path, PathBuf};
use std::time::Instant;
use xray_error::{XRayError, XRayResult};
use xray_utils::{assert, assert_equal};

pub struct PackEquipmentProcessor {}

impl PackEquipmentProcessor {
  pub fn pack_sprites(options: PackEquipmentOptions) -> XRayResult<PackEquipmentResult> {
    let started_at: Instant = Instant::now();

    let mut count: u32 = 0;
    let mut skipped_sections: Vec<&str> = Vec::new();
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
      InventorySpriteDescriptor::create_equipment_sprite_base_for_ltx(&options.ltx)?;

    for (section_name, section) in &options.ltx.sections {
      let Some(sprite_descriptor) =
        InventorySpriteDescriptor::new_optional_from_section(section_name, section)
      else {
        continue;
      };

      let Some((sprite_path, sprite)) = Self::read_sprite(&options, &sprite_descriptor) else {
        skipped_sections.push(section_name);
        continue;
      };

      let (x, y, w, h) = sprite_descriptor.get_boundaries();

      xray_output::verbose!(
        options.output,
        "Packing icon: '{}':({}:{};{}x{}) as ({}:{};{}x{}), src: {}x{}, {}",
        sprite_descriptor.section,
        sprite_descriptor.x,
        sprite_descriptor.y,
        sprite_descriptor.w,
        sprite_descriptor.h,
        x,
        y,
        w,
        h,
        sprite.width(),
        sprite.height(),
        sprite_path.display(),
      );

      image.copy_from(&sprite, x, y)?;
      count += 1;
    }

    Self::assert_every_section_has_an_icon(&options, &skipped_sections)?;

    assert_equal(
      image.width() % 4,
      0,
      "DirectX compression requires texture width to be multiple of 4",
    )?;
    assert_equal(
      image.height() % 4,
      0,
      "DirectX compression requires texture height to be multiple of 4",
    )?;

    save_image_as_ui_dds(&options.output_path, &image, options.dds_compression_format)?;

    xray_output::info!(
      options.output,
      "Packed {} icons in {} format",
      count,
      options.dds_compression_format
    );

    Ok(PackEquipmentResult {
      duration: started_at.elapsed().as_millis(),
      saved_at: options.output_path.clone(),
      saved_width: image.width(),
      saved_height: image.height(),
      packed_count: count,
    })
  }

  /// Fail once with every section that declares inventory grid coordinates but has no icon to pack.
  fn assert_every_section_has_an_icon(
    options: &PackEquipmentOptions,
    skipped_sections: &[&str],
  ) -> XRayResult {
    if !options.is_strict || skipped_sections.is_empty() {
      return Ok(());
    }

    Err(XRayError::new_texture_processing_error(format!(
      "Expected an icon to exist for each of the {} sections declaring inv_grid_* fields, found none for: {}",
      skipped_sections.len(),
      skipped_sections.join(", ")
    )))
  }

  pub fn read_sprite(
    options: &PackEquipmentOptions,
    sprite: &InventorySpriteDescriptor,
  ) -> Option<(PathBuf, DynamicImage)> {
    let (_, _, w, h) = sprite.get_boundaries();
    let sprite_path: PathBuf = Self::read_sprite_path(options, sprite);

    match Self::read_sprite_from_path(&sprite_path, w, h) {
      Ok(icon) => Some((sprite_path, icon)),
      Err(error) => {
        xray_output::warning!(
          options.output,
          "Skip icon {} / '{}', reason: {}",
          sprite_path.display(),
          sprite.section,
          error
        );

        None
      }
    }
  }

  /// Read rescaled png or dds icon to inject into one large equipment file.
  pub fn read_sprite_from_path(path: &Path, width: u32, height: u32) -> XRayResult<DynamicImage> {
    let image: DynamicImage = if path
      .extension()
      .is_some_and(|extension| extension.eq("png"))
    {
      ImageReader::open(path)?.decode()?
    } else {
      dds_to_image(&read_dds_by_path(path)?)?.into()
    };

    let image_width: u32 = image.width();
    let image_height: u32 = image.height();

    if image_width != width || image_height != height {
      log::info!(
        "Rescaling image to bounds: {}x{} from {}x{} {}",
        width,
        height,
        image_width,
        image_height,
        path.display()
      );

      let rescaled_image: DynamicImage = image.resize(width, height, FilterType::Lanczos3);
      let rescaled_width: u32 = rescaled_image.width();
      let rescaled_height: u32 = rescaled_image.height();

      if rescaled_width != width || rescaled_height != height {
        log::info!(
          "Re-center rescaled image to bounds: {}x{} from {}x{} {}",
          width,
          height,
          rescaled_width,
          rescaled_height,
          path.display()
        );

        let mut centered: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

        assert(
          rescaled_width <= width,
          "Unexpected width {rescaled_width} > {width} when rescaling",
        )?;
        assert(
          rescaled_height <= height,
          "Unexpected height {rescaled_height} > {height} when rescaling",
        )?;

        centered.copy_from(
          &rescaled_image,
          (width - rescaled_width) / 2,
          (height - rescaled_height) / 2,
        )?;

        Ok(centered.into())
      } else {
        Ok(rescaled_image)
      }
    } else {
      Ok(image)
    }
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
