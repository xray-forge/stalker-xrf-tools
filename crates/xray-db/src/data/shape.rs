use crate::types::{Matrix3d, Sphere3d};
use ini::Ini;

/// Shape enumeration stored in alife objects descriptors.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
  Sphere(Sphere3d),
  Box(Matrix3d),
}

impl Shape {
  pub fn export_shapes(shapes: &Vec<Shape>, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("shapes_count", shapes.len().to_string());

    for (index, shape) in shapes.iter().enumerate() {
      let prefix: String = format!("shape.{index}");

      match shape {
        Shape::Sphere(sphere) => {
          ini
            .with_section(Some(section))
            .set(format!("{prefix}.type"), "sphere")
            .set(format!("{prefix}.center"), sphere.0.to_string())
            .set(format!("{prefix}.radius"), sphere.1.to_string());
        }
        Shape::Box(square) => {
          ini
            .with_section(Some(section))
            .set(format!("{prefix}.type"), "box")
            .set(format!("{prefix}.a"), square.0.to_string())
            .set(format!("{prefix}.b"), square.1.to_string())
            .set(format!("{prefix}.c"), square.2.to_string())
            .set(format!("{prefix}.d"), square.3.to_string());
        }
      }
    }
  }
}
