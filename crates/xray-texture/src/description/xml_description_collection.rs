use std::collections::HashMap;
use std::fs;
use std::fs::{File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};

use roxmltree::{Document, Node, ParsingOptions};
use xray_error::{XRayError, XRayResult};

use crate::constants::{XML_ATTRIBUTE_ID, XML_ATTRIBUTE_NAME, XML_TAG_FILE, XML_TAG_TEXTURE, XML_TAG_WINDOW};
use crate::data::texture_file_descriptor::TextureFileDescriptor;
use crate::data::texture_sprite_descriptor::TextureSpriteDescriptor;
use crate::description::pack_description_options::PackDescriptionOptions;

pub struct XmlDescriptionCollection {
  pub files: HashMap<String, TextureFileDescriptor>,
}

impl XmlDescriptionCollection {
  /// Narrow the described files down to the ones requested by name, in the order requested.
  ///
  /// A description usually names several sheets, and packing rewrites every one of them. Selecting by
  /// name keeps a change to a single sheet from touching its neighbours. An unknown name is an error
  /// rather than a silently empty run, because that is almost always a typo.
  pub fn select_files(&self, options: &PackDescriptionOptions) -> XRayResult<Vec<&TextureFileDescriptor>> {
    if options.files.is_empty() {
      return Ok(self.files.values().collect());
    }

    let mut selected: Vec<&TextureFileDescriptor> = Vec::new();
    let mut unknown: Vec<&str> = Vec::new();

    for name in &options.files {
      let matched: Vec<&TextureFileDescriptor> = self
        .files
        .values()
        .filter(|it| Self::is_file_named(&it.name, name))
        .collect();

      match matched.len() {
        1 => selected.push(matched[0]),
        0 => unknown.push(name),
        _ => {
          return Err(XRayError::new_texture_processing_error(format!(
            "Expected '{}' to name a single described file, it matches {}",
            name,
            matched
              .iter()
              .map(|it| it.name.as_str())
              .collect::<Vec<&str>>()
              .join(", ")
          )));
        }
      }
    }

    if !unknown.is_empty() {
      let mut available: Vec<&str> = self.files.values().map(|it| it.name.as_str()).collect();
      available.sort_unstable();

      return Err(XRayError::new_texture_processing_error(format!(
        "Expected requested files to be described in {}, not found: {}, available: {}",
        options.description.display(),
        unknown.join(", "),
        available.join(", ")
      )));
    }

    Ok(selected)
  }

  /// Whether a described file name refers to the requested one.
  ///
  /// Described names carry their directory, as in `ui\ui_actor_weapons`, which is awkward to type on a
  /// command line. The bare file name is accepted too, and the separator may be given either way.
  fn is_file_named(described: &str, requested: &str) -> bool {
    let normalize = |value: &str| value.replace('\\', "/").to_lowercase();

    let described: String = normalize(described);
    let requested: String = normalize(requested);

    described == requested || described.rsplit('/').next().is_some_and(|base| base == requested)
  }

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
          let descriptions: HashMap<String, TextureFileDescriptor> = Self::get_description(options, &path)?;

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
    xray_output::verbose!(options.output, "Found texture description: {}", path.display());

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

        xray_output::warning!(options.output, "Error parsing XML file: {} - {}", path.display(), error);
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
          xray_output::warning!(options.output, "Invalid file node supplied without name attribute");
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
  use std::fs;
  use std::path::PathBuf;

  use image_dds::ImageFormat;

  use super::XmlDescriptionCollection;
  use crate::PackDescriptionOptions;

  #[test]
  fn returns_an_error_for_invalid_xml_in_strict_mode() {
    let path: PathBuf =
      std::env::temp_dir().join(format!("xray-texture-invalid-description-{}.xml", std::process::id()));

    let options: PackDescriptionOptions = PackDescriptionOptions {
      description: path.clone(),
      base: PathBuf::new(),
      output: Default::default(),
      output_path: PathBuf::new(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      files: Vec::new(),
      is_strict: true,
      is_parallel: false,
    };

    fs::write(&path, "<w>").unwrap();

    let result = XmlDescriptionCollection::get_description(&options, &path);

    fs::remove_file(&path).unwrap();

    assert!(result.is_err());
  }
}

#[cfg(test)]
mod select_files_tests {
  use std::collections::HashMap;
  use std::path::PathBuf;

  use image_dds::ImageFormat;

  use super::XmlDescriptionCollection;
  use crate::data::texture_file_descriptor::TextureFileDescriptor;
  use crate::description::pack_description_options::PackDescriptionOptions;

  fn collection_of(names: &[&str]) -> XmlDescriptionCollection {
    let mut files: HashMap<String, TextureFileDescriptor> = HashMap::new();

    for name in names {
      files.insert(String::from(*name), TextureFileDescriptor::new(*name));
    }

    XmlDescriptionCollection { files }
  }

  fn options_for(files: &[&str]) -> PackDescriptionOptions {
    PackDescriptionOptions {
      description: PathBuf::from("ui_actor_upgrades.xml"),
      base: PathBuf::new(),
      output: Default::default(),
      output_path: PathBuf::new(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      files: files.iter().map(|it| String::from(*it)).collect(),
      is_strict: false,
      is_parallel: false,
    }
  }

  #[test]
  fn selects_every_file_when_none_requested() {
    let collection: XmlDescriptionCollection = collection_of(&[r"ui\ui_actor_weapons", r"ui\ui_actor_armor"]);

    assert_eq!(
      collection
        .select_files(&options_for(&[]))
        .expect("expect all files to be selected")
        .len(),
      2
    );
  }

  #[test]
  fn selects_by_bare_file_name() {
    let collection: XmlDescriptionCollection = collection_of(&[r"ui\ui_actor_weapons", r"ui\ui_actor_armor"]);

    let selected = collection
      .select_files(&options_for(&["ui_actor_weapons"]))
      .expect("expect the bare name to resolve");

    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].name, r"ui\ui_actor_weapons");
  }

  #[test]
  fn selects_by_full_name_with_either_separator() {
    let collection: XmlDescriptionCollection = collection_of(&[r"ui\ui_actor_weapons"]);

    for requested in [r"ui\ui_actor_weapons", "ui/ui_actor_weapons"] {
      assert_eq!(
        collection
          .select_files(&options_for(&[requested]))
          .expect("expect either separator to resolve")
          .len(),
        1,
        "Expect {requested} to resolve"
      );
    }
  }

  #[test]
  fn rejects_an_unknown_name() {
    let collection: XmlDescriptionCollection = collection_of(&[r"ui\ui_actor_weapons"]);

    assert!(
      collection.select_files(&options_for(&["ui_typo"])).is_err(),
      "Expect an unknown name to fail rather than pack nothing"
    );
  }

  #[test]
  fn rejects_an_ambiguous_bare_name() {
    let collection: XmlDescriptionCollection = collection_of(&[r"ui\ui_actor_weapons", r"hud\ui_actor_weapons"]);

    assert!(
      collection.select_files(&options_for(&["ui_actor_weapons"])).is_err(),
      "Expect a bare name matching two described files to fail"
    );
  }
}
