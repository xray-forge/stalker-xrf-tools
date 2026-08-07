use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use xray_db::{OgfFile, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_output::OutputOptions;

#[derive(Default)]
pub struct PatchOgfMotionRefsCommand;

impl GenericCommand for PatchOgfMotionRefsCommand {
  fn name(&self) -> &'static str {
    "patch-ogf-motion-refs"
  }

  /// Create command for rewriting ogf motion refs.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to rewrite motion refs of provided ogf file")
      .arg(
        Arg::new("path")
          .help("Path to ogf file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("dest")
          .help("Path to resulting ogf file, defaults to in place rewrite of the source file")
          .short('d')
          .long("dest")
          .required(false)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("refs")
          .help("Motion refs to store in the ogf file")
          .short('r')
          .long("refs")
          .required(true)
          .num_args(1..)
          .value_parser(value_parser!(String)),
      )
      .arg(
        Arg::new("silent")
          .help("Disable any logging")
          .short('s')
          .long("silent")
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .short('v')
          .long("verbose")
          .action(ArgAction::SetTrue),
      )
  }

  /// Rewrite motion refs of provided ogf file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid input path to be provided");

    let motion_refs: Vec<String> = matches
      .get_many::<String>("refs")
      .expect("Expected valid motion refs to be provided")
      .cloned()
      .collect();

    let destination: &Path = matches
      .get_one::<PathBuf>("dest")
      .map_or(path.as_path(), |it| it.as_path());

    let output: OutputOptions =
      TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    Self::patch_file(&output, path, destination, &motion_refs)?;

    Ok(())
  }
}

impl PatchOgfMotionRefsCommand {
  /// Rewrite motion refs of single ogf file, preserving all other chunks byte for byte.
  fn patch_file(
    output: &OutputOptions,
    path: &Path,
    destination: &Path,
    motion_refs: &[String],
  ) -> XRayResult {
    let original: Vec<u8> = fs::read(path)?;
    let existing: Vec<String> = OgfFile::read_motion_refs_from_path::<XRayByteOrder, _>(&path)?;

    xray_output::info!(
      output,
      "Patch ogf motion refs {}, {:?} -> {:?}",
      path.display(),
      existing,
      motion_refs
    );

    Self::assert_chunk_copy_is_lossless(path, &original, &existing)?;

    let patched: Vec<u8> =
      OgfFile::write_motion_refs_to_buffer::<XRayByteOrder>(File::open(path)?, motion_refs)?;

    fs::write(destination, &patched)?;

    if let Err(error) = Self::assert_written_refs_match(destination, motion_refs) {
      Self::revert_destination(path, destination, &original)?;

      return Err(error);
    }

    xray_output::info!(
      output,
      "Ogf motion refs written into {}",
      destination.display()
    );

    Ok(())
  }

  /// Guard that rewriting the refs a file already has reproduces that file byte for byte.
  ///
  /// Geometry is not re-serializable, so this is what proves the chunk copy preserves everything
  /// outside the motion refs chunk. A mismatch means patching would silently corrupt the model.
  fn assert_chunk_copy_is_lossless(
    path: &Path,
    original: &[u8],
    existing: &[String],
  ) -> XRayResult {
    let reverted: Vec<u8> =
      OgfFile::write_motion_refs_to_buffer::<XRayByteOrder>(File::open(path)?, existing)?;

    if reverted != original {
      return Err(XRayError::new_verify_error(format!(
        "Refused to patch {}, rewriting its existing motion refs did not reproduce the source file, {} bytes original and {} bytes rewritten",
        path.display(),
        original.len(),
        reverted.len()
      )));
    }

    Ok(())
  }

  /// Guard that the written file reads back exactly the requested motion refs.
  fn assert_written_refs_match(destination: &Path, motion_refs: &[String]) -> XRayResult {
    let read_back: Vec<String> =
      OgfFile::read_motion_refs_from_path::<XRayByteOrder, _>(&destination)?;

    if read_back != motion_refs {
      return Err(XRayError::new_verify_error(format!(
        "Patched {} reads back motion refs {:?} instead of {:?}",
        destination.display(),
        read_back,
        motion_refs
      )));
    }

    Ok(())
  }

  /// Undo a failed write, leaving neither a corrupted source nor a partial destination behind.
  fn revert_destination(path: &Path, destination: &Path, original: &[u8]) -> XRayResult {
    if destination == path {
      fs::write(destination, original)?;
    } else {
      fs::remove_file(destination)?;
    }

    Ok(())
  }
}
