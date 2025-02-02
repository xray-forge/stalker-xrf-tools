use crate::export::file_import::read_ltx_field;
use crate::types::{Matrix3d, Sphere3d};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{
  ChunkReadable, ChunkReadableList, ChunkReader, ChunkWritable, ChunkWritableList, ChunkWriter,
};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::assert_equal;

/// Shape enumeration stored in objects descriptors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
  Sphere(Sphere3d),
  Box(Matrix3d),
}

impl ChunkReadable for Shape {
  /// Read shape from the chunk reader.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let shape_type: u8 = reader.read_u8().expect("Shape type to be read");

    Ok(match shape_type {
      0 => Self::Sphere((reader.read_xr::<T, _>()?, reader.read_f32::<T>()?)),
      1 => Self::Box((
        reader.read_xr::<T, _>()?,
        reader.read_xr::<T, _>()?,
        reader.read_xr::<T, _>()?,
        reader.read_xr::<T, _>()?,
      )),
      _ => {
        return Err(XRayError::new_parsing_error(
          "Unexpected shape type provided",
        ))
      }
    })
  }
}

impl ChunkReadableList for Shape {
  /// Read list of shapes from the chunk reader.
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Vec<Self>> {
    let mut shapes: Vec<Self> = Vec::new();
    let count: u8 = reader.read_u8().expect("Count flag to be read");

    for _ in 0..count {
      shapes.push(Self::read::<T>(reader)?);
    }

    assert_equal(
      shapes.len(),
      count as usize,
      "Declared and read shapes count should be equal",
    )?;

    Ok(shapes)
  }
}

impl ChunkWritable for Shape {
  /// Write shape data into the chunk reader.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    match self {
      Self::Sphere(data) => {
        writer.write_u8(0)?;

        data.0.write::<T>(writer)?;

        writer.write_f32::<T>(data.1)?;
      }
      Self::Box(data) => {
        writer.write_u8(1)?;

        data.0.write::<T>(writer)?;
        data.1.write::<T>(writer)?;
        data.2.write::<T>(writer)?;
        data.3.write::<T>(writer)?;
      }
    }

    Ok(())
  }
}

impl ChunkWritableList for Shape {
  /// Write list of shapes data into the chunk reader.
  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, shapes: &[Self]) -> XRayResult {
    writer.write_u8(shapes.len() as u8)?;

    for shape in shapes {
      shape.write::<T>(writer)?;
    }

    Ok(())
  }
}

impl Shape {
  /// Import shape objects from ltx config file.
  pub fn import_list(section: &Section) -> XRayResult<Vec<Self>> {
    let mut shapes: Vec<Self> = Vec::new();
    let count: usize = read_ltx_field("shapes_count", section)?;

    for index in 0..count {
      let prefix: String = format!("shape.{index}");
      let shape_type: String = read_ltx_field(&format!("{prefix}.type"), section)?;

      match shape_type.as_str() {
        "sphere" => {
          shapes.push(Self::Sphere((
            read_ltx_field(&format!("{prefix}.center"), section)?,
            read_ltx_field(&format!("{prefix}.radius"), section)?,
          )));
        }
        "box" => {
          shapes.push(Self::Box((
            read_ltx_field(&format!("{prefix}.a"), section)?,
            read_ltx_field(&format!("{prefix}.b"), section)?,
            read_ltx_field(&format!("{prefix}.c"), section)?,
            read_ltx_field(&format!("{prefix}.d"), section)?,
          )));
        }
        _ => {
          return Err(XRayError::new_parsing_error(format!(
            "Failed to parsed unknown type of shape - {shape_type} when importing from ltx"
          )))
        }
      }
    }

    Ok(shapes)
  }

  /// Export shapes object to target ltx file section.
  pub fn export_list(shapes: &[Self], section_name: &str, ltx: &mut Ltx) {
    ltx
      .with_section(section_name)
      .set("shapes_count", shapes.len().to_string());

    for (index, shape) in shapes.iter().enumerate() {
      let prefix: String = format!("shape.{index}");

      match shape {
        Self::Sphere(sphere) => {
          ltx
            .with_section(section_name)
            .set(format!("{prefix}.type"), "sphere")
            .set(format!("{prefix}.center"), sphere.0.to_string())
            .set(format!("{prefix}.radius"), sphere.1.to_string());
        }
        Self::Box(square) => {
          ltx
            .with_section(section_name)
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

#[cfg(test)]
mod tests {
  use crate::data::generic::shape::Shape;
  use crate::data::generic::vector_3d::Vector3d;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_chunk::{
    ChunkReadable, ChunkReadableList, ChunkReader, ChunkWritable, ChunkWritableList, ChunkWriter,
    XRayByteOrder,
  };
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write_list() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_list.chunk");

    let original: Vec<Shape> = vec![
      Shape::Sphere((
        Vector3d {
          x: 125.465,
          y: 456.123,
          z: 675.345,
        },
        150.0,
      )),
      Shape::Box((
        Vector3d {
          x: 10.5,
          y: 10.7,
          z: 10.0,
        },
        Vector3d {
          x: 20.5,
          y: 20.7,
          z: 20.0,
        },
        Vector3d {
          x: 30.5,
          y: 30.7,
          z: 30.0,
        },
        Vector3d {
          x: 40.5,
          y: 40.7,
          z: 40.0,
        },
      )),
    ];

    Shape::write_list::<XRayByteOrder>(&mut writer, &original)?;

    assert_eq!(writer.bytes_written(), 67);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 67);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 67 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(Shape::read_list::<XRayByteOrder>(&mut reader)?, original);

    Ok(())
  }

  #[test]
  fn test_read_write_sphere() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_sphere.chunk");

    let original: Shape = Shape::Sphere((
      Vector3d {
        x: 25.5,
        y: 3.4,
        z: 45.1,
      },
      150.0,
    ));

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 17);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 17);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 17 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(Shape::read::<XRayByteOrder>(&mut reader)?, original);

    Ok(())
  }

  #[test]
  fn test_read_write_box() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_box.chunk");

    let original: Shape = Shape::Box((
      Vector3d {
        x: 1.5,
        y: 1.7,
        z: 1.0,
      },
      Vector3d {
        x: 2.5,
        y: 2.7,
        z: 2.0,
      },
      Vector3d {
        x: 3.5,
        y: 3.7,
        z: 3.0,
      },
      Vector3d {
        x: 4.5,
        y: 4.7,
        z: 4.0,
      },
    ));

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 49);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 49);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 49 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(Shape::read::<XRayByteOrder>(&mut reader)?, original);

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "test_import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: Vec<Shape> = vec![
      Shape::Sphere((
        Vector3d {
          x: 1634.465,
          y: 2652.123,
          z: 3624.345,
        },
        150.0,
      )),
      Shape::Box((
        Vector3d {
          x: 1000.5,
          y: 1000.7,
          z: 1000.0,
        },
        Vector3d {
          x: 2000.5,
          y: 2000.7,
          z: 2000.0,
        },
        Vector3d {
          x: 3000.5,
          y: 3000.7,
          z: 3000.0,
        },
        Vector3d {
          x: 4000.5,
          y: 4000.7,
          z: 4000.0,
        },
      )),
    ];

    Shape::export_list(&original, "data", &mut ltx);
    ltx.write_to(&mut overwrite_file(config_path)?)?;

    assert_eq!(
      Shape::import_list(Ltx::read_from_path(config_path)?.section("data").unwrap(),)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_sphere() -> XRayResult {
    let original: Shape = Shape::Sphere((
      Vector3d {
        x: 243.5,
        y: 456.4,
        z: 475.1,
      },
      52.0,
    ));

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize_sphere.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<Shape>(&serialized).unwrap()
    );

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_box() -> XRayResult {
    let original: Shape = Shape::Box((
      Vector3d {
        x: 175.5,
        y: 135.7,
        z: 163.0,
      },
      Vector3d {
        x: 264.5,
        y: 274.7,
        z: 244.0,
      },
      Vector3d {
        x: 375.5,
        y: 385.7,
        z: 386.0,
      },
      Vector3d {
        x: 498.5,
        y: 460.7,
        z: 489.0,
      },
    ));

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize_box.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<Shape>(&serialized).unwrap()
    );

    Ok(())
  }
}
