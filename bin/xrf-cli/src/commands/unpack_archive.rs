use clap::ArgMatches;
use std::env;
use std::path::PathBuf;
use xray_archive::{ArchiveProject, ArchiveUnpackResult};

/// Unpack xray engine database archive.
pub async fn unpack_archive(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid path to be provided");

  let mut destination: PathBuf = matches
    .get_one::<PathBuf>("dest")
    .expect("Expected valid output path to be provided")
    .clone();

  let parallel: usize = *matches
    .get_one::<usize>("parallel")
    .expect("Expected valid parallel threads count to be provided");

  if destination.is_relative() {
    destination = env::current_dir().unwrap().join(destination);
  }

  log::info!("Unpack source: {:?}", path);
  log::info!("Unpack destination: {:?}", destination);

  let archive_project: ArchiveProject = ArchiveProject::new(path).unwrap();

  log::info!(
    "Summary: {} archive(s), {} file(s), {:.3} MB compressed, {:.3} MB real",
    archive_project.archives.len(),
    archive_project.files.len(),
    (archive_project.get_compressed_size() as f64) / 1024.0 / 1024.0,
    (archive_project.get_real_size() as f64) / 1024.0 / 1024.0,
  );

  log::info!("Unpacking files, parallel {parallel}");

  let result: ArchiveUnpackResult = archive_project
    .unpack_parallel(&destination, parallel)
    .await
    .unwrap();

  log::info!(
    "Unpacked archive, took {} sec (preparation {} sec, unpack {} sec)",
    result.duration as f64 / 1000.0,
    result.prepare_duration as f64 / 1000.0,
    result.unpack_duration as f64 / 1000.0,
  )
}
