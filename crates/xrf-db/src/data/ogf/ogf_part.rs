use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xrf_chunk::{ChunkReadWrite, ChunkReadWriteList, ChunkReader, ChunkWriter};
use xrf_error::{XrfError, XrfResult};
use xrf_utils::assert_length;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfPart {
  pub name: String,
  pub bones: Vec<(String, u32)>, // name + index.
}

impl OgfPart {
  pub fn get_bones(&self) -> Vec<&str> {
    self.bones.iter().map(|it| it.0.as_str()).collect::<Vec<_>>()
  }
}

impl ChunkReadWriteList for OgfPart {
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> XrfResult<Vec<Self>> {
    let count: u16 = reader.read_u16::<T>()?;
    let mut parts: Vec<Self> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      parts.push(
        Self::read::<T>(reader)
          .map_err(|error| XrfError::new_read_error(format!("Failed to read ogf part: {error}")))?,
      );
    }

    assert_length(&parts, count as usize, "Expected correct count of OGF parts to be read")?;

    Ok(parts)
  }

  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, parts: &[Self]) -> XrfResult {
    writer.write_u16::<T>(parts.len() as u16)?;

    for part in parts {
      part
        .write::<T>(writer)
        .map_err(|error| XrfError::new_serialization_error(format!("Failed to write ogf part: {error}")))?;
    }

    Ok(())
  }
}

impl ChunkReadWrite for OgfPart {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XrfResult<Self> {
    let name: String = reader.read_w1251_string()?;
    let count: u16 = reader.read_u16::<T>()?;

    let mut bones: Vec<(String, u32)> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      bones.push((reader.read_w1251_string()?, reader.read_u32::<T>()?));
    }

    Ok(Self { name, bones })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XrfResult {
    writer.write_w1251_string(&self.name)?;
    writer.write_u16::<T>(self.bones.len() as u16)?;

    for (name, index) in &self.bones {
      writer.write_w1251_string(name)?;
      writer.write_u32::<T>(*index)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use xrf_chunk::{ChunkReadWriteList, ChunkReader, ChunkWriter, XRayByteOrder};
  use xrf_error::XrfResult;
  use xrf_test_utils::FileSlice;
  use xrf_test_utils::utils::{
    get_relative_test_sample_file_path, open_generated_test_resource_as_slice,
    overwrite_generated_test_resource_as_file,
  };

  use crate::data::ogf::ogf_part::OgfPart;

  #[test]
  fn test_read_write_list() -> XrfResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_list.chunk");

    let original: Vec<OgfPart> = vec![
      OgfPart {
        name: String::from("default"),
        bones: vec![(String::from("bip01"), 0)],
      },
      OgfPart {
        name: String::from("right_hand"),
        bones: vec![(String::from("r_hand"), 1), (String::from("lead_gun"), 2)],
      },
    ];

    OgfPart::write_list::<XRayByteOrder>(&mut writer, &original)?;

    writer.flush_chunk_into::<XRayByteOrder>(&mut overwrite_generated_test_resource_as_file(&filename)?, 0)?;

    let file: FileSlice = open_generated_test_resource_as_slice(&filename)?;
    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(OgfPart::read_list::<XRayByteOrder>(&mut reader)?, original);

    Ok(())
  }
}
