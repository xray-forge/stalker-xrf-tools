use clap::ArgMatches;
use std::path::PathBuf;

pub fn pack_texture_description(matches: &ArgMatches) {
  let description: &PathBuf = matches
    .get_one::<PathBuf>("description")
    .expect("Expected valid path to be provided for texture description file or folder");

  let base: &PathBuf = matches
    .get_one::<PathBuf>("base")
    .expect("Expected valid base path to be provided");

  let output: &PathBuf = matches.get_one::<PathBuf>("output").unwrap_or(base);

  let is_verbose: bool = matches.get_flag("verbose");

  log::info!("Packing texture descriptions from: {:?}", description);
}
