use crate::data::texture_file_descriptor::TextureFileDescriptor;
use crate::description::xml_description_collection::XmlDescriptionCollection;
use crate::{
  dds_to_image, read_dds_by_path, save_image_as_ui_dds, PackDescriptionOptions, TextureResult,
};
use image::{GenericImageView, RgbaImage};
use rayon::prelude::*;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct UnpackDescriptionProcessor {}

impl UnpackDescriptionProcessor {
  pub fn unpack_xml_descriptions(options: PackDescriptionOptions) -> TextureResult<()> {
    let description: XmlDescriptionCollection =
      XmlDescriptionCollection::get_descriptions(&options)?;
    let count: Mutex<u32> = Mutex::new(0);

    println!("Unpacking for {:?} files", description.files.len());

    if options.is_parallel {
      println!("Unpacking for {:?} files", description.files.len());

      description.files.par_iter().for_each(|(_, file)| {
        if Self::unpack_xml_description(&options, file).is_ok_and(|it| it) {
          *count.lock().unwrap() += 1;
        }
      });
    } else {
      for file in description.files.values() {
        if Self::unpack_xml_description(&options, file)? {
          *count.lock().unwrap() += 1;
        }
      }
    }

    println!("Unpacked {} files", *count.lock().unwrap());

    Ok(())
  }

  pub fn unpack_xml_description(
    options: &PackDescriptionOptions,
    file: &TextureFileDescriptor,
  ) -> TextureResult<bool> {
    let full_name: PathBuf = options.base.join(format!("{}.dds", file.name));
    let destination: PathBuf = options.output.join(&file.name);

    if options.is_verbose {
      println!("Unpacking {:?}", full_name);
    }

    let dds: TextureResult<RgbaImage> =
      read_dds_by_path(&full_name).and_then(|dds| dds_to_image(&dds));

    if let Ok(dds) = dds {
      if !destination.exists() {
        create_dir_all(&destination)?;
      }

      for sprite in &file.sprites {
        if options.is_verbose {
          println!("Unpacking {:?} -> {}", full_name, sprite.id);
        }

        let (max_x, max_y) = sprite.get_dimension_boundaries();

        if max_x > dds.width() || max_y > dds.height() {
          if options.is_strict {
            panic!(
              "Unexpected texture '{}' (x:{max_x}, y:{max_y}) boundaries are bigger than source DDS file ({}x{} - {:?})",
              sprite.id,
              dds.width(),
              dds.height(),
              full_name
            );
          } else {
            println!(
              "[WARN] - exceeding sprite size '{}' (x:{max_x}, y:{max_y}) ({}x{} - {:?})",
              sprite.id,
              dds.width(),
              dds.height(),
              full_name
            );
          }
        } else {
          save_image_as_ui_dds(
            &destination.join(format!("{}.dds", sprite.id)),
            &dds.view(sprite.x, sprite.y, sprite.w, sprite.h).to_image(),
            options.dds_compression_format,
          )?;
        }
      }

      Ok(true)
    } else if options.is_strict {
      panic!("Could not find file for texture unpacking: {:?}", full_name)
    } else {
      println!(
        "Skip file {:?}, not able to read: {:?}",
        full_name,
        dds.unwrap_err().to_string()
      );

      Ok(false)
    }
  }
}
