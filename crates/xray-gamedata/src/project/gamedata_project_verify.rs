use crate::{GamedataProject, GamedataResult};

impl GamedataProject {
  pub fn verify(&self) -> GamedataResult<bool> {
    log::info!(
      "Verifying gamedata project: {:?} | {:?}",
      self.roots,
      self.configs
    );

    println!("Verify gamedata");

    let format_result: GamedataResult = self.verify_ltx_format();
    let schemes_result: GamedataResult = self.verify_ltx_schemes();
    let weapons_result: GamedataResult = self.verify_ltx_weapons();

    Ok(format_result.is_ok() && schemes_result.is_ok() && weapons_result.is_ok())
  }
}
