use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_weathers(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataWeathersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata weathers (todo):".green(),);
    }

    Ok(GamedataWeathersVerificationResult {})
  }
}
