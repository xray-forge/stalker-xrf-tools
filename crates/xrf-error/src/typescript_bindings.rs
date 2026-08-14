use specta::Types;

pub fn typescript_bindings() -> Types {
  Types::default().register::<crate::XrfError>()
}
