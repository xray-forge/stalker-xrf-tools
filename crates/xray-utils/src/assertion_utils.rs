use std::fmt::Debug;
use xray_error::{XRayError, XRayResult};

/// Assert condition.
pub fn assert(condition: bool, message: &str) -> XRayResult {
  if condition {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(message))
  }
}

/// Assert data values are equal.
pub fn assert_equal<T: PartialEq + Debug>(first: T, second: T, message: &str) -> XRayResult {
  if first == second {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(format!(
      "Expected values to be equal, left - {:?}, right - {:?}. {message}",
      first, second
    )))
  }
}

/// Assert data values are not equal.
pub fn assert_not_equal<T: PartialEq + Debug>(first: T, second: T, message: &str) -> XRayResult {
  if first != second {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(format!(
      "Expected values not to be equal, left - {:?}, right - {:?}. {message}",
      first, second
    )))
  }
}
