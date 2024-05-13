use clap::ArgMatches;
use std::path::PathBuf;
use xray_icon::{pack_xml_descriptions, ImageFormat, PackDescriptionOptions};

pub fn pack_texture_description(matches: &ArgMatches) {
  let description: &PathBuf = matches
    .get_one::<PathBuf>("description")
    .expect("Expected valid path to be provided for texture description file or folder");

  let base: &PathBuf = matches
    .get_one::<PathBuf>("base")
    .expect("Expected valid base path to be provided");

  let output: &PathBuf = matches.get_one::<PathBuf>("output").unwrap_or(base);

  let is_verbose: bool = matches.get_flag("verbose");
  let is_strict: bool = matches.get_flag("strict");

  let options: PackDescriptionOptions = PackDescriptionOptions {
    description: description.clone(),
    base: base.clone(),
    output: output.clone(),
    dds_compression_format: ImageFormat::BC3RgbaUnorm,
    is_verbose,
    is_strict,
  };

  log::info!("Packing texture descriptions from: {:?}", description);
  log::info!("Paths: base {:?}, output {:?}", base, output);
  log::info!("DDS format: {:?}", options.dds_compression_format);

  pack_xml_descriptions(options)
}
