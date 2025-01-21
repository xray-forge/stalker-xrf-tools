pub trait GamedataGenericVerificationResult {
  fn is_valid(&self) -> bool;

  fn get_failure_message(&self) -> String;
}
