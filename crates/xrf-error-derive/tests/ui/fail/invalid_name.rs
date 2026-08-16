use xrf_error_derive::ErrorConstructors;

#[derive(ErrorConstructors)]
enum ExampleError {
  #[constructor("not valid")]
  Invalid { message: String },
}

fn main() {}
