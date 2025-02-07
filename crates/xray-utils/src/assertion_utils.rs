use std::fmt::Debug;
use xray_error::{XRayError, XRayResult};

/// Assert condition.
#[inline]
pub fn assert(condition: bool, message: &str) -> XRayResult {
  if condition {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(message))
  }
}

/// Assert data values are equal.
#[inline]
pub fn assert_length<T>(container: &[T], expected_len: usize, message: &str) -> XRayResult {
  if container.len() == expected_len {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(format!(
      "Expected container size to match value, actual size - {}, expected - {}. {}",
      container.len(),
      expected_len,
      message
    )))
  }
}

/// Assert data values are equal.
#[inline]
pub fn assert_equal<T: PartialEq + Debug>(first: T, second: T, message: &str) -> XRayResult {
  if first == second {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(format!(
      "Expected values to be equal, left - {:?}, right - {:?}. {}",
      first, second, message
    )))
  }
}

/// Assert data values are not equal.
#[inline]
pub fn assert_not_equal<T: PartialEq + Debug>(first: T, second: T, message: &str) -> XRayResult {
  if first != second {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(format!(
      "Expected values not to be equal, left - {:?}, right - {:?}. {}",
      first, second, message
    )))
  }
}
