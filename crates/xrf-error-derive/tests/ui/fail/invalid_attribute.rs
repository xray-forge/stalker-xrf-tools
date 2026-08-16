use xrf_error_derive::ErrorConstructors;

#[derive(ErrorConstructors)]
enum ExampleError {
  #[constructor = "custom_error"]
  Invalid { message: String },
}

fn main() {}
