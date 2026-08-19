use specta::Types;

pub fn typescript_bindings() -> Types {
  Types::default()
    .register::<crate::ArchiveExtractResult>()
    .register::<crate::ArchiveExtractDirectoryResult>()
    .register::<crate::ArchiveUnpackResult>()
    .register::<crate::ArchivePackResult>()
    .register::<crate::ArchivePackConfig>()
}
