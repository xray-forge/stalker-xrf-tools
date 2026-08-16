use xrf_error_derive::ErrorConstructors;

#[derive(ErrorConstructors)]
struct ExampleError {
  message: String,
}

fn main() {}
