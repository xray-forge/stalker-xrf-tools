use crate::types::{Matrix3d, Sphere3d};

pub enum Shape {
  Sphere(Sphere3d),
  Box(Matrix3d),
}
