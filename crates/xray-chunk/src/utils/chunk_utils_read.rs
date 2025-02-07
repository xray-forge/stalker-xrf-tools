use crate::ChunkReader;
use byteorder::{ByteOrder, ReadBytesExt};
use xray_error::XRayResult;

/// Read chunk as u16 value, verify remaining data is 0.
#[inline]
pub fn read_u16_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<u16> {
  let data: u16 = reader.read_u16::<T>()?;

  reader.assert_read("All data should be read in u16 chunk")?;

  Ok(data)
}

/// Read chunk as u32 value, verify remaining data is 0.
#[inline]
pub fn read_u32_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<u32> {
  let data: u32 = reader.read_u32::<T>()?;

  reader.assert_read("All data should be read in u32 chunk")?;

  Ok(data)
}

/// Read chunk as f32 value, verify remaining data is 0.
#[inline]
pub fn read_f32_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<f32> {
  let data: f32 = reader.read_f32::<T>()?;

  reader.assert_read("All data should be read in f32 chunk")?;

  Ok(data)
}

/// Read chunk as f32 vector value, verify remaining data is 0.
#[inline]
pub fn read_f32_vector_chunk<T: ByteOrder>(
  reader: &mut ChunkReader,
) -> XRayResult<(f32, f32, f32)> {
  let data: (f32, f32, f32) = (
    reader.read_f32::<T>()?,
    reader.read_f32::<T>()?,
    reader.read_f32::<T>()?,
  );

  reader.assert_read("All data should be read in f32 vector chunk")?;

  Ok(data)
}

/// Read chunk as containing string, verify remaining data is 0.
#[inline]
pub fn read_w1251_string_chunk(reader: &mut ChunkReader) -> XRayResult<String> {
  let data: String = reader.read_w1251_string()?;

  reader.assert_read("All data should be read in string chunk")?;

  Ok(data)
}
