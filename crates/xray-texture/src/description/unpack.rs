use crate::{read_dds_by_path, save_image_as_ui_dds, PackDescriptionOptions};
use image::{GenericImageView, RgbaImage};
use image_dds::ImageFormat;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;

use crate::description::file_description::FileDescription;
use crate::description::xml_description::get_files_descriptions;

pub fn unpack_xml_descriptions(options: PackDescriptionOptions) {
  let files: HashMap<String, FileDescription> = get_files_descriptions(&options);
  let mut count: u32 = 0;

  println!("Unpacking for {:?} files", files.len());

  for file in files.values() {
    let is_unpacked: bool = unpack_xml_description(&options, file);

    if is_unpacked {
      count += 1;
    }
  }

  println!("Unpacked {count} files");
}

pub fn unpack_xml_description(options: &PackDescriptionOptions, file: &FileDescription) -> bool {
  let full_name: PathBuf = options.base.join(format!("{}.dds", file.name));
  let destination: PathBuf = options.output.join(&file.name);

  if options.is_verbose {
    println!("Unpacking {:?}", full_name);
  }

  let dds: io::Result<RgbaImage> = read_dds_by_path(&full_name);

  if let Ok(dds) = dds {
    if !destination.exists() {
      create_dir_all(&destination).unwrap();
    }

    for texture in &file.textures {
      if options.is_verbose {
        println!("Unpacking {:?} -> {}", full_name, texture.id);
      }

      if texture.x + texture.w > dds.width() || texture.y + texture.h > dds.height() {
        if options.is_strict {
          panic!(
            "Unexpected texture {} boundaries are bigger than source DDS file ({}x{} - {:?})",
            texture.id,
            dds.width(),
            dds.height(),
            full_name
          );
        } else {
          println!(
            "[WARN] - exceeding size of sprite file for {} ({}x{} - {:?})",
            texture.id,
            dds.width(),
            dds.height(),
            full_name
          );

          save_image_as_ui_dds(
            &destination.join(format!("{}.dds", texture.id)),
            &dds
              .view(
                texture.x,
                texture.y,
                if texture.x + texture.w <= dds.width() {
                  texture.w
                } else {
                  dds.width() - texture.x
                },
                if texture.y + texture.h <= dds.width() {
                  texture.h
                } else {
                  dds.height() - texture.y
                },
              )
              .to_image(),
            ImageFormat::BC3RgbaUnorm,
          );
        }
      } else {
        save_image_as_ui_dds(
          &destination.join(format!("{}.dds", texture.id)),
          &dds
            .view(texture.x, texture.y, texture.w, texture.h)
            .to_image(),
          ImageFormat::BC3RgbaUnorm,
        );
      }
    }

    true
  } else if options.is_strict {
    panic!("Could not find file for texture unpacking: {:?}", full_name)
  } else {
    println!("Skip file {:?}, not able to read", full_name);

    false
  }
}
