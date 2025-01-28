use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use ddsfile::Dds;
use std::fs::File;
use std::path::PathBuf;

#[derive(Default)]
pub struct InfoDdsCommand;

impl GenericCommand for InfoDdsCommand {
  fn name(&self) -> &'static str {
    "info-dds"
  }

  /// Create command for printing texture info.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to print information about provided dds file")
      .arg(
        Arg::new("path")
          .help("Path to dds file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Print information about dds file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    println!("Read dds file {}", path.display());

    let mut dds_file: File = File::open(path)?;
    let dds: Box<Dds> = Box::new(Dds::read(&mut dds_file)?);

    let file_size: u64 = dds_file.metadata()?.len();
    let data_size: usize = dds.data.len();

    println!("File size: {} ({}kb)", file_size, file_size / 1024);
    println!("Metadata size: {} ", file_size - data_size as u64);
    println!("Data size: {} ({}kb)", data_size, data_size / 1024);

    println!("Size: {} x {}", dds.header.width, dds.header.height,);

    println!(
      "Mipmap: {} - {}",
      dds.get_num_mipmap_levels(),
      dds.get_min_mipmap_size_in_bytes(),
    );

    if let Some(depth) = dds.header.depth {
      println!("Depth: {}", depth);
    }

    if let Some(pitch) = dds.header.pitch {
      println!("Pitch: {}", pitch);
    }

    if let Some(linear_size) = dds.header.linear_size {
      println!("Linear size: {}", linear_size);
    }

    if let Some(format) = dds.get_format() {
      if let Some(block_size) = format.get_block_size() {
        println!("Block size: {}", block_size);
      }

      if let Some(bits_per_pixel) = format.get_bits_per_pixel() {
        println!("Bits per pixel: {}", bits_per_pixel);
      }

      if let Some(four_cc) = format.get_fourcc() {
        println!("Four CC: {}", four_cc.0);
      }
    } else {
      println!("Format: unknown");
    }

    if let Some(d3d_format) = dds.get_d3d_format() {
      println!("D3D format: {:?}", d3d_format);
    } else if let Some(dxgi_format) = dds.get_dxgi_format() {
      println!("DXGI format: {:?}", dxgi_format);
    }

    Ok(())
  }
}
