//! X-Ray LZHUF: the LZSS plus adaptive Huffman codec behind `_compressLZ` and `_decompressLZ`.
//!
//! X-Ray marks a chunk id with bit 31 to say its payload is coded this way, and stores the payload as a
//! little-endian `u32` decompressed size followed by the bit stream. Archive file descriptors are the
//! most visible user; the engine applies the same coding to any chunk written through `IWriter::w_chunk`.
//!
//! The coding is Okumura's LZHUF with one X-Ray deviation, `REBUILD_FREQUENCY_LIMIT`: the adaptive tree
//! is rebuilt at half the frequency standard LHA `-lh1-` uses. That single constant is why a stock LZH
//! library decodes the first 16384 symbols of an X-Ray stream and then diverges, and why XRF owns this
//! code.
//!
//! Streams arrive whole in memory, so the API takes and returns slices rather than readers.

mod bit_reader;
mod bit_writer;
mod decode;
mod encode;
mod lzhuf_constants;
mod match_distance;
mod ring_buffer;
#[cfg(test)]
mod tests;
mod tree;

pub use crate::decode::lzhuf_decode::{decompress, decompress_into};
pub use crate::encode::lzhuf_encode::compress;
