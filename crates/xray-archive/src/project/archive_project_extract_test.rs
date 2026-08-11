use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crc32fast::hash;

use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
use crate::project::archive_project_read_policy::ArchiveProjectReadPolicy;
use crate::{ArchiveExtractFolderResult, ArchiveProject};

struct Entry {
  name: &'static str,
  contents: &'static [u8],
}

/// Lay out entries end to end in one file and describe them, the way an archive stores its payload.
fn create_project(directory: &Path, entries: &[Entry]) -> ArchiveProject {
  let source: PathBuf = directory.join("files.db0");

  let mut payload: Vec<u8> = Vec::new();
  let mut files: HashMap<String, ArchiveFileDescriptor> = HashMap::new();

  for entry in entries {
    let offset: u32 = payload.len() as u32;

    payload.extend_from_slice(entry.contents);

    let mut descriptor: ArchiveFileDescriptor = ArchiveFileDescriptor::new(
      hash(entry.contents),
      entry.name.into(),
      offset,
      entry.contents.len() as u32,
      entry.contents.len() as u32,
    );

    descriptor.source = source.clone();

    files.insert(entry.name.into(), descriptor);
  }

  fs::File::create(&source)
    .expect("test archive file")
    .write_all(&payload)
    .expect("test archive payload");

  ArchiveProject {
    archives: Vec::new(),
    files,
    read_policy: ArchiveProjectReadPolicy::default(),
    root: directory.into(),
    size_real: payload.len() as u64,
  }
}

fn create_temporary_directory(name: &str) -> PathBuf {
  let directory: PathBuf = std::env::temp_dir().join(format!("xray-archive-extract-{name}"));

  let _ = fs::remove_dir_all(&directory);

  fs::create_dir_all(&directory).expect("temporary directory");

  directory
}

#[test]
fn extract_folder_writes_every_file_under_the_prefix() {
  let directory: PathBuf = create_temporary_directory("folder");
  let project: ArchiveProject = create_project(
    &directory,
    &[
      Entry {
        name: "configs\\gameplay\\dialogs.xml",
        contents: b"<game_dialogs/>",
      },
      Entry {
        name: "configs\\system.ltx",
        contents: b"[section]",
      },
      Entry {
        name: "meshes\\actor.ogf",
        contents: b"ogf",
      },
    ],
  );

  let out: PathBuf = directory.join("out");
  let result: ArchiveExtractFolderResult = project.extract_folder("configs", &out).expect("extraction");

  assert_eq!(result.extracted_count, 2);
  // The prefix is stripped: the user chose the destination for that folder already.
  assert_eq!(
    fs::read_to_string(out.join("gameplay").join("dialogs.xml")).expect("nested file"),
    "<game_dialogs/>"
  );
  assert_eq!(
    fs::read_to_string(out.join("system.ltx")).expect("root file"),
    "[section]"
  );
  assert!(!out.join("actor.ogf").exists(), "must not reach outside the prefix");
}

#[test]
fn extract_folder_skips_entries_that_carry_no_bytes() {
  let directory: PathBuf = create_temporary_directory("empty");
  let project: ArchiveProject = create_project(
    &directory,
    &[
      // Archives contain zero length entries, and some of them name a directory. Opening one as a
      // file is what produced "the system cannot find the path specified".
      Entry {
        name: "configs\\gameplay\\",
        contents: b"",
      },
      Entry {
        name: "configs\\gameplay\\dialogs.xml",
        contents: b"<game_dialogs/>",
      },
    ],
  );

  let out: PathBuf = directory.join("out");
  let result: ArchiveExtractFolderResult = project.extract_folder("configs", &out).expect("extraction");

  assert_eq!(result.extracted_count, 1);
  assert!(out.join("gameplay").join("dialogs.xml").exists());
}

#[test]
fn extract_folder_takes_the_whole_archive_for_an_empty_prefix() {
  let directory: PathBuf = create_temporary_directory("root");
  let project: ArchiveProject = create_project(
    &directory,
    &[
      Entry {
        name: "configs\\system.ltx",
        contents: b"[section]",
      },
      Entry {
        name: "meshes\\actor.ogf",
        contents: b"ogf",
      },
    ],
  );

  let out: PathBuf = directory.join("out");
  let result: ArchiveExtractFolderResult = project.extract_folder("", &out).expect("extraction");

  assert_eq!(result.extracted_count, 2);
  assert!(out.join("configs").join("system.ltx").exists());
  assert!(out.join("meshes").join("actor.ogf").exists());
}

#[test]
fn extract_file_writes_to_the_exact_path_it_is_given() {
  let directory: PathBuf = create_temporary_directory("single");
  let project: ArchiveProject = create_project(
    &directory,
    &[Entry {
      name: "configs\\system.ltx",
      contents: b"[section]",
    }],
  );

  let target: PathBuf = directory.join("chosen").join("renamed.ltx");

  project
    .extract_file("configs\\system.ltx", &target)
    .expect("extraction");

  assert_eq!(fs::read_to_string(&target).expect("written file"), "[section]");
}

#[test]
fn read_file_bytes_returns_the_stored_contents() {
  let directory: PathBuf = create_temporary_directory("bytes");
  let project: ArchiveProject = create_project(
    &directory,
    &[
      Entry {
        name: "configs\\system.ltx",
        contents: b"[section]",
      },
      Entry {
        name: "textures\\wall.dds",
        contents: b"\x44\x44\x53\x20not-a-real-dds",
      },
    ],
  );

  // Reads by offset, so the second entry must not bleed into the first.
  assert_eq!(
    project.read_file_bytes("configs\\system.ltx").expect("bytes"),
    b"[section]"
  );
  assert_eq!(
    project.read_file_bytes("textures\\wall.dds").expect("bytes"),
    b"\x44\x44\x53\x20not-a-real-dds"
  );
}

#[test]
fn read_file_bytes_reports_an_unknown_name() {
  let directory: PathBuf = create_temporary_directory("bytes-missing");
  let project: ArchiveProject = create_project(
    &directory,
    &[Entry {
      name: "configs\\system.ltx",
      contents: b"[section]",
    }],
  );

  assert!(project.read_file_bytes("configs\\other.ltx").is_err());
}

#[test]
fn extract_folder_reports_a_prefix_that_matches_nothing() {
  let directory: PathBuf = create_temporary_directory("missing");
  let project: ArchiveProject = create_project(
    &directory,
    &[Entry {
      name: "configs\\system.ltx",
      contents: b"[section]",
    }],
  );

  assert!(project.extract_folder("meshes", directory.join("out")).is_err());
}
