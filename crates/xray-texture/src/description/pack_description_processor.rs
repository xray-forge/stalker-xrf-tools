use crate::data::texture_file_descriptor::TextureFileDescriptor;
use crate::description::pack_description_options::PackDescriptionOptions;
use crate::description::xml_description_collection::XmlDescriptionCollection;
use crate::{dds_to_image, read_dds_by_path, save_image_as_ui_dds, TextureResult};
use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use std::path::PathBuf;

pub struct PackDescriptionProcessor {}

impl PackDescriptionProcessor {
  /// Pack list of xml files by options.
  pub fn pack_xml_descriptions(options: &PackDescriptionOptions) -> TextureResult {
    let description: XmlDescriptionCollection =
      XmlDescriptionCollection::get_descriptions(options)?;
    let mut count: u32 = 0;

    println!("Packing for {:?} files", description.files.len());

    for file in description.files.values() {
      if Self::pack_xml_description(options, file)? {
        count += 1;
      }
    }

    println!("Packed {count} files");

    Ok(())
  }

  pub fn pack_xml_description(
    options: &PackDescriptionOptions,
    file: &TextureFileDescriptor,
  ) -> TextureResult<bool> {
    let full_name: PathBuf = options.base.join(format!("{}.dds", file.name));

    let (width, height) = file.get_dimension_boundaries();
    let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

    if options.is_verbose {
      println!("Packing file {:?} ({width}x{height})", full_name);
    }

    for texture in &file.sprites {
      if options.is_verbose {
        println!(
          "Packing texture {:?} -> {} at [x:{}, y:{}, w:{}, h:{}]",
          full_name, texture.id, texture.x, texture.y, texture.w, texture.h
        );
      }

      let texture_path: PathBuf = options
        .base
        .join(&file.name)
        .join(format!("{}.dds", texture.id));

      match read_dds_by_path(&texture_path).and_then(|dds| dds_to_image(&dds)) {
        Ok(texture_dds) => {
          assert_eq!(
            texture_dds.width(),
            texture.w,
            "XML file texture width and actual DDS size should match"
          );
          assert_eq!(
            texture_dds.height(),
            texture.h,
            "XML file texture height and actual DDS size should match"
          );

          result
            .copy_from(&texture_dds, texture.x, texture.y)
            .expect("Properly copied DDS texture into resulting file");
        }
        Err(error) => {
          if options.is_strict {
            panic!(
              "Failed to read texture dds {:?} for {:?} ({:?}): {:?}",
              texture.id, file.name, full_name, error
            )
          } else {
            println!(
              "Failed to read texture dds {:?} for {:?} ({:?}): {:?}",
              texture.id, file.name, full_name, error
            )
          }
        }
      }
    }

    let destination: PathBuf = options.output.join(format!("{}.dds", &file.name));

    if options.is_verbose {
      println!("Saving file: {:?}", destination);
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

    save_image_as_ui_dds(&destination, &result, options.dds_compression_format)?;

    Ok(true)
  }
}
