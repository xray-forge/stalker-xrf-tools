use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use std::time::Instant;

impl GamedataProject {
  pub fn verify_weathers(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataWeathersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata weathers (todo):".green(),);
    }

    let started_at: Instant = Instant::now();
    let duration: u128 = started_at.elapsed().as_millis();

    Ok(GamedataWeathersVerificationResult { duration })
  }
}
