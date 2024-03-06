use crate::{EscapePolicy, Ltx, LtxError, ParseOptions, WriteOptions};
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct LtxProject {
  pub root: PathBuf,
  pub entries: Vec<DirEntry>,
}

impl LtxProject {
  pub fn on_root(root: &Path) -> io::Result<LtxProject> {
    let mut entries: Vec<DirEntry> = Vec::new();

    for entry in WalkDir::new(root) {
      let value: DirEntry = entry?;

      if value.path().extension().is_some_and(|it| it == "ltx") {
        entries.push(value);
      }
    }

    Ok(LtxProject {
      root: PathBuf::from(root),
      entries,
    })
  }

  pub fn format_all(&self) -> Result<(), LtxError> {
    for entry in &self.entries {
      println!("Formatting: {:?}", entry.path());

      let ltx: Ltx = Ltx::load_from_file_full_opt(
        entry.path(),
        ParseOptions {
          enabled_escape: false,
          enabled_quote: false,
        },
      )?;

      let mut destination = PathBuf::from("target/assets");

      destination.push(entry.file_name());

      ltx
        .write_to_file_opt(
          destination,
          WriteOptions {
            escape_policy: EscapePolicy::Nothing,
            ..Default::default()
          },
        )
        .unwrap();
    }

    println!("Formatted {} ltx files", self.entries.len());

    Ok(())
  }
}
