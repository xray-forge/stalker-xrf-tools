use specta::Types;

pub fn typescript_bindings() -> Types {
  Types::default()
    .register::<crate::ArchiveDescriptor>()
    .register::<crate::ArchiveFileDescriptor>()
    .register::<crate::ArchiveProject>()
    .register::<crate::ArchiveProjectReadPolicy>()
    .register::<crate::ProjectReadResult>()
    .register::<crate::XrayAssetLocation>()
}
