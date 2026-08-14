use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use image::{GenericImageView, RgbaImage};
use image_dds::Mipmaps;
use rayon::prelude::*;
use xrf_error::{XrfError, XrfResult};

use crate::constants::DDS_EXTENSION;
use crate::data::texture_file_descriptor::TextureFileDescriptor;
use crate::description::xml_description_collection::XmlDescriptionCollection;
use crate::{PackDescriptionOptions, dds_to_image, read_dds_by_path, save_image_as_ui_dds};

pub struct UnpackDescriptionProcessor {}

impl UnpackDescriptionProcessor {
  pub fn unpack_xml_descriptions(options: PackDescriptionOptions) -> XrfResult<()> {
    let description: XmlDescriptionCollection = XmlDescriptionCollection::get_descriptions(&options)?;
    let count: AtomicU32 = AtomicU32::new(0);
    let selected: Vec<&TextureFileDescriptor> = description.select_files(&options)?;

    xrf_output::info!(options.output, "Unpacking for {} files", selected.len());

    if options.is_parallel {
      selected.par_iter().try_for_each(|file| {
        if Self::unpack_xml_description(&options, file)? {
          count.fetch_add(1, Ordering::Relaxed);
        }

        Ok::<(), XrfError>(())
      })?;
    } else {
      for file in selected {
        if Self::unpack_xml_description(&options, file)? {
          count.fetch_add(1, Ordering::Relaxed);
        }
      }
    }

    xrf_output::info!(options.output, "Unpacked {} files", count.load(Ordering::Relaxed));

    Ok(())
  }

  pub fn unpack_xml_description(options: &PackDescriptionOptions, file: &TextureFileDescriptor) -> XrfResult<bool> {
    let full_name: PathBuf = options.base.join(format!("{}.{}", file.name, DDS_EXTENSION));
    let destination: PathBuf = options.output_path.join(&file.name);

    xrf_output::verbose!(options.output, "Unpacking {}", full_name.display());

    let dds: XrfResult<RgbaImage> = read_dds_by_path(&full_name).and_then(|dds| dds_to_image(&dds));

    if let Ok(dds) = dds {
      if !destination.exists() {
        create_dir_all(&destination)?;
      }

      for sprite in &file.sprites {
        xrf_output::verbose!(options.output, "Unpacking {} -> {}", full_name.display(), sprite.id);

        let (max_x, max_y) = sprite.get_dimension_boundaries();

        if max_x > dds.width() || max_y > dds.height() {
          if options.is_strict {
            return Err(XrfError::new_texture_processing_error(format!(
              "Unexpected texture '{}' (x:{}, y:{}) boundaries are bigger than source DDS file ({}x{} - {})",
              sprite.id,
              max_x,
              max_y,
              dds.width(),
              dds.height(),
              full_name.display()
            )));
          } else {
            xrf_output::warning!(
              options.output,
              "[WARN] - exceeding sprite size '{}' (x:{}, y:{}) ({}x{} - {})",
              sprite.id,
              max_x,
              max_y,
              dds.width(),
              dds.height(),
              full_name.display()
            );
          }
        } else {
          // Unpacked sprites are packing input read at their base level, so a mip chain would only
          // cost space.
          save_image_as_ui_dds(
            &destination.join(format!("{}.{}", sprite.id, DDS_EXTENSION)),
            &dds.view(sprite.x, sprite.y, sprite.w, sprite.h).to_image(),
            options.dds_compression_format,
            Mipmaps::Disabled,
          )?;
        }
      }

      Ok(true)
    } else if options.is_strict {
      Err(XrfError::new_texture_processing_error(format!(
        "Could not find file for texture unpacking: {}",
        full_name.display()
      )))
    } else {
      xrf_output::warning!(
        options.output,
        "Skip file {}, not able to read: {}",
        full_name.display(),
        dds.unwrap_err()
      );

      Ok(false)
    }
  }
}
