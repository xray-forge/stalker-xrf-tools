use xrf_error_derive::ErrorConstructors;

#[derive(Debug, ErrorConstructors, PartialEq)]
enum ExampleError {
  #[constructor]
  Automatic { message: String },
  #[constructor("custom_error")]
  Custom { message: String },
  Ignored,
}

fn main() {
  assert_eq!(
    ExampleError::new_automatic_error("automatic"),
    ExampleError::Automatic {
      message: String::from("automatic"),
    }
  );
  assert_eq!(
    ExampleError::custom_error(String::from("custom")),
    ExampleError::Custom {
      message: String::from("custom"),
    }
  );
  let _ = ExampleError::Ignored;
}
