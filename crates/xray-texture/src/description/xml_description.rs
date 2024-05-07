use crate::description::file_description::FileDescription;
use crate::description::texture_description::TextureDescription;
use crate::UnpackDescriptionOptions;
use roxmltree::{Document, Node, ParsingOptions};
use std::fs;
use std::fs::{File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn get_files_descriptions(options: &UnpackDescriptionOptions) -> Vec<FileDescription> {
  if options.description.is_dir() {
    println!(
      "Check texture descriptions from dir: {:?}",
      options.description
    );

    let mut files: Vec<FileDescription> = Vec::new();
    let entries: ReadDir = fs::read_dir(&options.description).unwrap();

    for entry in entries.flatten() {
      let path: PathBuf = entry.path();

      if let Some(extension) = path.extension() {
        if extension == "xml" {
          let descriptions: Vec<FileDescription> = get_file_descriptions(options, &path);

          files.extend(descriptions);
        }
      }
    }

    files
  } else {
    get_file_descriptions(options, &options.description)
  }
}

pub fn get_file_descriptions(
  options: &UnpackDescriptionOptions,
  path: &Path,
) -> Vec<FileDescription> {
  if options.is_verbose {
    println!("Found texture description: {:?}", path);
  }

  let mut descriptions: Vec<FileDescription> = Vec::new();

  let mut file: File = File::open(path).unwrap();
  let mut text: String = String::new();

  file.read_to_string(&mut text).unwrap();

  let document: Document = match Document::parse_with_options(
    &text,
    ParsingOptions {
      allow_dtd: true,
      ..ParsingOptions::default()
    },
  ) {
    Ok(doc) => doc,
    Err(error) => {
      println!("Error: {}.", error);
      return Vec::new();
    }
  };

  let window: Option<Node> = document
    .root()
    .children()
    .find(|it| it.is_element() && it.tag_name().name().eq("w"));

  if let Some(window) = window {
    for file in window
      .children()
      .filter(|it| it.is_element() && it.tag_name().name().eq("file"))
    {
      let file_name: Option<&str> = file.attribute("name");

      if let Some(file_name) = file_name {
        if options.is_verbose {
          println!("Parsing file: {file_name}");
        }

        let mut file_description: FileDescription = FileDescription::new(file_name);

        for texture in file
          .descendants()
          .filter(|it| it.is_element() && it.tag_name().name().eq("texture"))
        {
          let id: Option<&str> = texture.attribute("id");
          let x: Option<&str> = texture.attribute("x");
          let y: Option<&str> = texture.attribute("y");
          let w: Option<&str> = texture.attribute("width");
          let h: Option<&str> = texture.attribute("height");

          if id.is_none() || x.is_none() || x.is_none() || x.is_none() || x.is_none() {
            println!("Skip texture: {:?} {:?} {:?} {:?} {:?}", id, x, y, w, h);
          } else {
            let id: &str = id.unwrap();
            let x: u32 = x
              .unwrap()
              .trim()
              .parse::<u32>()
              .expect("Correct 'x' attribute");
            let y: u32 = y
              .unwrap()
              .trim()
              .parse::<u32>()
              .expect("Correct 'y' attribute");
            let w: u32 = w
              .unwrap()
              .trim()
              .parse::<u32>()
              .expect("Correct 'width' attribute");
            let h: u32 = h
              .unwrap()
              .trim()
              .parse::<u32>()
              .expect("Correct 'height' attribute");

            if options.is_verbose {
              println!(
                "Parsed texture: id:{:?}, x:{:?}, y:{:?}, w:{:?}, h:{:?}",
                id, x, y, w, h
              );
            }

            file_description.add_texture(TextureDescription::new(id, x, y, w, h));
          }
        }

        if file_description.textures.is_empty() {
          println!("Skip definitions {file_name} without textures");
        } else {
          descriptions.push(file_description)
        }
      } else {
        println!("Invalid file node supplied without name attribute");
      }
    }
  } else {
    println!("Got no 'w' tag for file '{:?}'", options.description);
  }

  descriptions
}
