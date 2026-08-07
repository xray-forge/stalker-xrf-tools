use crate::constants::{
  XML_ATTRIBUTE_ID, XML_ATTRIBUTE_NAME, XML_TAG_FILE, XML_TAG_TEXTURE, XML_TAG_WINDOW,
};
use crate::data::texture_file_descriptor::TextureFileDescriptor;
use crate::data::texture_sprite_descriptor::TextureSpriteDescriptor;
use crate::description::pack_description_options::PackDescriptionOptions;
use roxmltree::{Document, Node, ParsingOptions};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};
use xray_error::{XRayError, XRayResult};

pub struct XmlDescriptionCollection {
  pub files: HashMap<String, TextureFileDescriptor>,
}

impl XmlDescriptionCollection {
  /// Get descriptions from provided options.
  /// Handle both directory and single file as inputs.
  pub fn get_descriptions(options: &PackDescriptionOptions) -> XRayResult<Self> {
    if options.description.is_dir() {
      xray_output::info!(
        options.output,
        "Check texture descriptions from dir: {}",
        options.description.display()
      );

      let mut files: HashMap<String, TextureFileDescriptor> = HashMap::new();
      let entries: ReadDir = fs::read_dir(&options.description)?;

      for entry in entries.flatten() {
        let path: PathBuf = entry.path();

        if let Some(extension) = path.extension()
          && extension == "xml"
        {
          let descriptions: HashMap<String, TextureFileDescriptor> =
            Self::get_description(options, &path)?;

          descriptions
            .into_iter()
            .for_each(|(name, description)| match files.get_mut(&name) {
              None => {
                files.insert(name, description);
              }
              Some(existing) => {
                xray_output::verbose!(options.output, "Merging textures for {name}");

                existing.sprites.extend(description.sprites);
              }
            })
        }
      }

      Ok(Self { files })
    } else {
      Ok(Self {
        files: Self::get_description(options, &options.description)?,
      })
    }
  }

  /// Get descriptions from provided file path.
  pub fn get_description(
    options: &PackDescriptionOptions,
    path: &Path,
  ) -> XRayResult<HashMap<String, TextureFileDescriptor>> {
    xray_output::verbose!(
      options.output,
      "Found texture description: {}",
      path.display()
    );

    let mut descriptions: HashMap<String, TextureFileDescriptor> = HashMap::new();

    let mut file: File = File::open(path)?;
    let mut text: String = String::new();

    file.read_to_string(&mut text)?;

    let document: Document = match Document::parse_with_options(
      &text,
      ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
      },
    ) {
      Ok(doc) => doc,
      Err(error) => {
        if options.is_strict {
          return Err(XRayError::new_parsing_error(format!(
            "Failed to parse xml: {} - {}",
            path.display(),
            error
          )));
        }

        xray_output::warning!(
          options.output,
          "Error parsing XML file: {} - {}",
          path.display(),
          error
        );
        return Ok(HashMap::new());
      }
    };

    let window: Option<Node> = document
      .root()
      .children()
      .find(|it| it.is_element() && it.tag_name().name().eq(XML_TAG_WINDOW));

    if let Some(window) = window {
      for file in window
        .children()
        .filter(|it| it.is_element() && it.tag_name().name().eq(XML_TAG_FILE))
      {
        let file_name: Option<&str> = file.attribute(XML_ATTRIBUTE_NAME);

        if let Some(file_name) = file_name {
          xray_output::verbose!(options.output, "Parsing file: {file_name}");

          let mut file_description: TextureFileDescriptor = TextureFileDescriptor::new(file_name);

          for node in file
            .descendants()
            .filter(|it| it.is_element() && it.tag_name().name().eq(XML_TAG_TEXTURE))
          {
            if let Some(sprite) = TextureSpriteDescriptor::new_optional_from_node(node) {
              file_description.add_sprite(sprite);
            } else {
              xray_output::warning!(
                options.output,
                "Skip texture node: {} ({})",
                node.attribute(XML_ATTRIBUTE_ID).unwrap_or("unknown"),
                node
                  .attributes()
                  .map(|it| format!("{}={}", it.name(), it.value()))
                  .collect::<Vec<String>>()
                  .join(","),
              );
            }
          }

          if file_description.sprites.is_empty() {
            xray_output::warning!(
              options.output,
              "Skip definitions node \"{file_name}\" without textures (in {})",
              path.display()
            );
          } else {
            match descriptions.get_mut(&file_description.name) {
              None => {
                descriptions.insert(file_description.name.clone(), file_description);
              }
              Some(existing) => {
                xray_output::verbose!(options.output, "Merging textures for {file_name}");

                file_description
                  .sprites
                  .into_iter()
                  .for_each(|it| existing.sprites.push(it));
              }
            }
          }
        } else {
          xray_output::warning!(
            options.output,
            "Invalid file node supplied without name attribute"
          );
        }
      }
    } else {
      xray_output::warning!(
        options.output,
        "Got no 'w' tag for file '{}'",
        options.description.display()
      );
    }

    Ok(descriptions)
  }
}

#[cfg(test)]
mod tests {
  use super::XmlDescriptionCollection;
  use crate::PackDescriptionOptions;
  use image_dds::ImageFormat;
  use std::fs;
  use std::path::PathBuf;

  #[test]
  fn returns_an_error_for_invalid_xml_in_strict_mode() {
    let path: PathBuf = std::env::temp_dir().join(format!(
      "xray-texture-invalid-description-{}.xml",
      std::process::id()
    ));

    let options: PackDescriptionOptions = PackDescriptionOptions {
      description: path.clone(),
      base: PathBuf::new(),
      output: Default::default(),
      output_path: PathBuf::new(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      is_strict: true,
      is_parallel: false,
    };

    fs::write(&path, "<w>").unwrap();

    let result = XmlDescriptionCollection::get_description(&options, &path);

    fs::remove_file(&path).unwrap();

    assert!(result.is_err());
  }
}
