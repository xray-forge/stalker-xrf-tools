use crate::spawn::types::{U8v4, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};
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
pub fn read_u8v4(file: &mut FileSlice) -> U8v4 {
  (
    file.read_u8().unwrap(),
    file.read_u8().unwrap(),
    file.read_u8().unwrap(),
    file.read_u8().unwrap(),
  )
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
