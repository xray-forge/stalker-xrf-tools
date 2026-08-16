use xrf_error_derive::ErrorConstructors;

#[derive(ErrorConstructors)]
enum ExampleError {
  #[constructor(not_a_string)]
  Invalid { message: String },
}

fn main() {}
