use crate::{GamedataProject, GamedataResult};
use xray_ltx::{LtxFormatOptions, LtxVerifyOptions};

impl GamedataProject {
  pub fn verify_ltx_format(&self) -> GamedataResult {
    println!("Verify gamedata LTX format");

    self
      .ltx_project
      .check_format_all_files_opt(LtxFormatOptions { is_silent: false })?;

    Ok(())
  }

  pub fn verify_ltx_schemes(&self) -> GamedataResult {
    println!("Verify gamedata LTX schemas");

    self.ltx_project.verify_entries_opt(LtxVerifyOptions {
      is_silent: false,
      is_verbose: false,
      is_strict: false,
    })?;

    Ok(())
  }
}
