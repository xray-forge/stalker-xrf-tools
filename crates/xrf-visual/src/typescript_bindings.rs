use specta::Types;

pub fn typescript_bindings() -> Types {
  Types::default()
    .register::<crate::VisualBounds>()
    .register::<crate::VisualBox>()
    .register::<crate::VisualSphere>()
    .register::<crate::VisualBone>()
    .register::<crate::VisualDescription>()
    .register::<crate::VisualDrawRange>()
    .register::<crate::VisualGeometry>()
    .register::<crate::VisualSection>()
    .register::<crate::VisualSkipCause>()
    .register::<crate::VisualSlideWindow>()
    .register::<crate::VisualSubmesh>()
    .register::<crate::VisualSubmeshContent>()
}
