use crate::export::file_import::read_ini_field;
use crate::types::{Matrix3d, Sphere3d};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

/// Shape enumeration stored in alife objects descriptors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
  Sphere(Sphere3d),
  Box(Matrix3d),
}

impl Shape {
  /// Import shape objects from ini config file.
  pub fn import_shapes(section: &Section) -> io::Result<Vec<Shape>> {
    let mut shapes: Vec<Shape> = Vec::new();
    let count: usize = read_ini_field("shapes_count", section)?;

    for index in 0..count {
      let prefix: String = format!("shape.{index}");
      let shape_type: String = read_ini_field(&format!("{prefix}.type"), section)?;

      match shape_type.as_str() {
        "sphere" => {
          shapes.push(Shape::Sphere((
            read_ini_field(&format!("{prefix}.center"), section)?,
            read_ini_field(&format!("{prefix}.radius"), section)?,
          )));
        }
        "box" => {
          shapes.push(Shape::Box((
            read_ini_field(&format!("{prefix}.a"), section)?,
            read_ini_field(&format!("{prefix}.b"), section)?,
            read_ini_field(&format!("{prefix}.c"), section)?,
            read_ini_field(&format!("{prefix}.d"), section)?,
          )));
        }
        _ => {
          return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Failed to parsed unknown type shape - {shape_type}"),
          ))
        }
      }
    }

    Ok(shapes)
  }

  /// Export shapes object to target ini file section.
  pub fn export_shapes(shapes: &[Shape], section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("shapes_count", shapes.len().to_string());

    for (index, shape) in shapes.iter().enumerate() {
      let prefix: String = format!("shape.{index}");

      match shape {
        Shape::Sphere(sphere) => {
          ini
            .with_section(section)
            .set(format!("{prefix}.type"), "sphere")
            .set(format!("{prefix}.center"), sphere.0.to_string())
            .set(format!("{prefix}.radius"), sphere.1.to_string());
        }
        Shape::Box(square) => {
          ini
            .with_section(section)
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
