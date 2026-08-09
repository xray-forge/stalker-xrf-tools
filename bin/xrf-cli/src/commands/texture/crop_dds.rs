use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use xray_output::OutputOptions;
use xray_texture::{
  DynamicImage, GenericImageView, ImageFormat, Mipmaps, PNG_EXTENSION, RgbaImage, dds_to_image,
  fit_image_into_bounds, read_dds_by_path, save_image_as_ui_dds, save_image_as_ui_png,
};

use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;

#[derive(Default)]
pub struct CropDdsCommand;

impl GenericCommand for CropDdsCommand {
  fn name(&self) -> &'static str {
    "crop-dds"
  }

  /// Create command for cropping a region out of a dds file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to crop a rectangular region out of a dds file into a new dds file")
      .arg(
        Arg::new("source")
          .help("Path to the dds file to read the region from")
          .long("source")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("output")
          .help("Path of the dds file to write")
          .long("output")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("x")
          .help("Left edge of the region, in pixels")
          .long("x")
          .required(true)
          .value_parser(value_parser!(u32)),
      )
      .arg(
        Arg::new("y")
          .help("Top edge of the region, in pixels")
          .long("y")
          .required(true)
          .value_parser(value_parser!(u32)),
      )
      .arg(
        Arg::new("width")
          .help("Width of the region, in pixels")
          .long("width")
          .required(true)
          .value_parser(value_parser!(u32)),
      )
      .arg(
        Arg::new("height")
          .help("Height of the region, in pixels")
          .long("height")
          .required(true)
          .value_parser(value_parser!(u32)),
      )
      .arg(
        Arg::new("fit-width")
          .help("Scale the cropped region to this width, preserving aspect and letterboxing")
          .long("fit-width")
          .requires("fit-height")
          .value_parser(value_parser!(u32)),
      )
      .arg(
        Arg::new("fit-height")
          .help("Scale the cropped region to this height, preserving aspect and letterboxing")
          .long("fit-height")
          .requires("fit-width")
          .value_parser(value_parser!(u32)),
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

  /// Crop a region out of a dds file, optionally scaling it into different bounds.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let source: &PathBuf = matches
      .get_one::<PathBuf>("source")
      .expect("Expected valid source path to be provided");
    let output_path: &PathBuf = matches
      .get_one::<PathBuf>("output")
      .expect("Expected valid output path to be provided");

    let x: u32 = *matches.get_one::<u32>("x").expect("Expected valid x");
    let y: u32 = *matches.get_one::<u32>("y").expect("Expected valid y");
    let width: u32 = *matches
      .get_one::<u32>("width")
      .expect("Expected valid width");
    let height: u32 = *matches
      .get_one::<u32>("height")
      .expect("Expected valid height");

    let output: OutputOptions =
      TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    let image: RgbaImage = dds_to_image(&read_dds_by_path(source)?)?;

    if x + width > image.width() || y + height > image.height() {
      return Err(
        format!(
          "Region {x}:{y} {width}x{height} does not fit in source {} which is {}x{}",
          source.display(),
          image.width(),
          image.height()
        )
        .into(),
      );
    }

    let cropped: RgbaImage = image.view(x, y, width, height).to_image();

    let result: RgbaImage = match (
      matches.get_one::<u32>("fit-width"),
      matches.get_one::<u32>("fit-height"),
    ) {
      (Some(fit_width), Some(fit_height)) => {
        xray_output::info!(
          output,
          "Fitting cropped {}x{} region into {}x{}",
          width,
          height,
          fit_width,
          fit_height
        );

        fit_image_into_bounds(DynamicImage::from(cropped), *fit_width, *fit_height, source)?.into()
      }
      _ => cropped,
    };

    // Writing png keeps the region lossless, which matters when it becomes a packing source.
    if output_path
      .extension()
      .is_some_and(|extension| extension.eq(PNG_EXTENSION))
    {
      save_image_as_ui_png(output_path, &result)?;
    } else {
      save_image_as_ui_dds(
        output_path,
        &result,
        ImageFormat::BC3RgbaUnorm,
        // A cropped region is packing input read at its base level, so a mip chain would only cost
        // space.
        Mipmaps::Disabled,
      )?;
    }

    xray_output::info!(
      output,
      "Wrote {}x{} region from {}:{} of {} to {}",
      result.width(),
      result.height(),
      x,
      y,
      source.display(),
      output_path.display()
    );

    Ok(())
  }
}
