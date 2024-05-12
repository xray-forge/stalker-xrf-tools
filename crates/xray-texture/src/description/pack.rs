use crate::description::dimensions::get_xml_description_sprite_max_dimension;
use crate::description::file_description::FileDescription;
use crate::description::pack_options::PackDescriptionOptions;
use crate::description::xml_description::get_files_descriptions;
use crate::{read_dds_by_path, save_image_as_ui_dds};
use image::{GenericImage, ImageBuffer, Rgba, RgbaImage};
use image_dds::ImageFormat;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn pack_xml_descriptions(options: PackDescriptionOptions) {
  let files: HashMap<String, FileDescription> = get_files_descriptions(&options);
  let mut count: u32 = 0;

  println!("Packing for {:?} files", files.len());

  for (_, file) in &files {
    let is_packed: bool = pack_xml_description(&options, file);

    if is_packed {
      count += 1;
    }
  }

  println!("Packed {count} files");
}

pub fn pack_xml_description(options: &PackDescriptionOptions, file: &FileDescription) -> bool {
  let full_name: PathBuf = options.base.join(format!("{}.dds", file.name));

  let (width, height) = get_xml_description_sprite_max_dimension(file);
  let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

  if options.is_verbose {
    println!("Packing file {:?} ({width}x{height})", full_name);
  }

  for texture in &file.textures {
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

    match read_dds_by_path(&texture_path) {
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

  save_image_as_ui_dds(&destination, &result, ImageFormat::BC3RgbaUnorm);

  true
}
