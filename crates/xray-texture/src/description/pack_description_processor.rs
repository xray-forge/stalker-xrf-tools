use std::path::PathBuf;

use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use xray_error::{XRayError, XRayResult};
use xray_utils::assert_equal;

use crate::constants::{DDS_EXTENSION, UI_MIPMAP_LEVELS, UI_MIPMAPS};
use crate::data::texture_file_descriptor::TextureFileDescriptor;
use crate::description::pack_description_options::PackDescriptionOptions;
use crate::description::xml_description_collection::XmlDescriptionCollection;
use crate::utils::images::warn_on_reshaped_ui_dds;
use crate::{dds_to_image, read_dds_by_path, save_image_as_ui_dds};

pub struct PackDescriptionProcessor {}

impl PackDescriptionProcessor {
  /// Pack list of xml files by options.
  pub fn pack_xml_descriptions(options: &PackDescriptionOptions) -> XRayResult {
    let description: XmlDescriptionCollection = XmlDescriptionCollection::get_descriptions(options)?;
    let mut count: u32 = 0;

    let selected: Vec<&TextureFileDescriptor> = description.select_files(options)?;

    xray_output::info!(options.output, "Packing for {} files", selected.len());

    for file in selected {
      if Self::pack_xml_description(options, file)? {
        count += 1;
      }
    }

    xray_output::info!(options.output, "Packed {count} files");

    Ok(())
  }

  pub fn pack_xml_description(options: &PackDescriptionOptions, file: &TextureFileDescriptor) -> XRayResult<bool> {
    let full_name: PathBuf = options.base.join(format!("{}.{}", file.name, DDS_EXTENSION));

    let (width, height) = file.get_dimension_boundaries();
    let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

    xray_output::verbose!(
      options.output,
      "Packing file {} ({width}x{height})",
      full_name.display()
    );

    for texture in &file.sprites {
      xray_output::verbose!(
        options.output,
        "Packing texture {} -> {} at [x:{}, y:{}, w:{}, h:{}]",
        full_name.display(),
        texture.id,
        texture.x,
        texture.y,
        texture.w,
        texture.h
      );

      let texture_path: PathBuf = options
        .base
        .join(&file.name)
        .join(format!("{}.{}", texture.id, DDS_EXTENSION));

      match read_dds_by_path(&texture_path).and_then(|dds| dds_to_image(&dds)) {
        Ok(texture_dds) => {
          assert_equal(
            texture_dds.width(),
            texture.w,
            "XML file texture width and actual DDS size should match",
          )?;
          assert_equal(
            texture_dds.height(),
            texture.h,
            "XML file texture height and actual DDS size should match",
          )?;

          result
            .copy_from(&texture_dds, texture.x, texture.y)
            .map_err(|error| XRayError::new_texture_processing_error(error.to_string()))?;
        }
        Err(error) => {
          if options.is_strict {
            return Err(XRayError::new_texture_processing_error(format!(
              "Failed to read texture dds {} for {} ({}): {}",
              texture.id,
              file.name,
              full_name.display(),
              error
            )));
          } else {
            xray_output::warning!(
              options.output,
              "Failed to read texture dds {} for {} ({}): {}",
              texture.id,
              file.name,
              full_name.display(),
              error
            )
          }
        }
      }
    }

    let destination: PathBuf = options.output_path.join(format!("{}.{}", &file.name, DDS_EXTENSION));

    xray_output::verbose!(options.output, "Saving file: {}", destination.display());

    warn_on_reshaped_ui_dds(&options.output, &destination, width, height, UI_MIPMAP_LEVELS);

    save_image_as_ui_dds(&destination, &result, options.dds_compression_format, UI_MIPMAPS)?;

    Ok(true)
  }
}
