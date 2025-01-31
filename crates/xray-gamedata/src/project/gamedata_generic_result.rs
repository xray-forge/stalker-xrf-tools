use xray_error::XRayResult;

pub trait GamedataGenericVerificationResult {
  fn is_valid(&self) -> bool;

  fn get_failure_message(&self) -> String;

  fn is_optional_ok_and_valid<T: GamedataGenericVerificationResult>(
    result: &Option<XRayResult<T>>,
  ) -> bool {
    result.as_ref().is_none_or(|it| {
      it.as_ref()
        .is_ok_and(GamedataGenericVerificationResult::is_valid)
    })
  }

  fn get_optional_result_failure_message<T>(
    result: &Option<XRayResult<T>>,
    comment: &str,
  ) -> Option<String>
  where
    T: GamedataGenericVerificationResult,
  {
    if let Some(result) = result {
      match result {
        Ok(result) => {
          if result.is_valid() {
            None
          } else {
            Some(result.get_failure_message())
          }
        }
        Err(error) => Some(format!("Check failed ({comment}): {error}")),
      }
    } else {
      None
    }
  }
}
