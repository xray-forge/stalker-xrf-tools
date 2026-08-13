use specta::Types;

pub fn typescript_bindings() -> Types {
  Types::default()
    .register::<crate::ExportsProject>()
    .register::<crate::ExportSourceContent>()
    .register::<crate::ExportDescriptor>()
    .register::<crate::ExportContractDescriptor>()
    .register::<crate::ExportParameterDescriptor>()
    .register::<crate::ExportReturnDescriptor>()
    .register::<crate::ExportSourceDescriptor>()
}
