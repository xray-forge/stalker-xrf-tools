use crate::spawn::types::{U32Bytes, Vector3d};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use fileslice::FileSlice;
use std::io::{Read, Seek, SeekFrom};

/// Read three float values.
pub fn read_f32_vector<T: ByteOrder>(file: &mut FileSlice) -> Vector3d<f32> {
  (
    file.read_f32::<T>().unwrap(),
    file.read_f32::<T>().unwrap(),
    file.read_f32::<T>().unwrap(),
  )
}

/// Read four bytes in natural order.
pub fn read_u32_bytes(file: &mut FileSlice) -> U32Bytes {
  (
    file.read_u8().unwrap(),
    file.read_u8().unwrap(),
    file.read_u8().unwrap(),
    file.read_u8().unwrap(),
  )
}

/// Read shape data.
pub fn read_shape_description(file: &mut FileSlice) -> Vec<f32> {
  let mut shape: Vec<f32> = Vec::new();
  let count: u8 = file.read_u8().unwrap();

  assert_eq!(count, 1, "Single shape description expected.");

  for _ in 0..count {
    let shape_type: u8 = file.read_u8().unwrap();

    match shape_type {
      0 => {
        for _ in 0..4 {
          shape.push(file.read_f32::<LittleEndian>().unwrap())
        }
      }
      1 => {
        for _ in 0..12 {
          shape.push(file.read_f32::<LittleEndian>().unwrap())
        }
      }
      _ => panic!("Unexpected shape type provided"),
    }
  }

  shape
}

/// Read null terminated string from file bytes.
pub fn read_null_terminated_string(file: &mut FileSlice) -> String {
  let offset: u64 = file.seek(SeekFrom::Current(0)).unwrap();
  let mut buffer: Vec<u8> = Vec::new();

  file.read_to_end(&mut buffer).unwrap();

  if let Some(position) = buffer.iter().position(|&x| x == 0x00) {
    let value: String =
      String::from_utf8(buffer[..position].to_vec()).expect("Correct string read.");

    // Put seek right after string - length plus zero terminator.
    file
      .seek(SeekFrom::Start(offset + value.len() as u64 + 1))
      .expect("Correct object seek movement.");

    return value;
  } else {
    panic!("No null terminator found in file");
  }
}
