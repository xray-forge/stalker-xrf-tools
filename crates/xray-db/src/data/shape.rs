use crate::types::{Matrix3d, Sphere3d};

/// Shape enumeration stored in alife objects descriptors.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
  Sphere(Sphere3d),
  Box(Matrix3d),
}
