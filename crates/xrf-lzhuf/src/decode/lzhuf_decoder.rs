use xrf_error::XrfResult;

use crate::bit_reader::BitReader;
use crate::lzhuf_constants::{MATCH_CODE_BASE, MATCH_LENGTH_THRESHOLD};
use crate::ring_buffer::RingBuffer;
use crate::tree::dynamic_huffman_tree::DynamicHuffmanTree;

/// Decodes one headerless X-Ray LZHUF bit stream.
///
/// The decoder is single-shot: it fills the caller's buffer and stops. A match that would run past the
/// end of that buffer is truncated rather than carried over, because the buffer length is the encoder's
/// own declared output size and nothing consumes bytes beyond it.
pub(crate) struct LzhufDecoder<'a> {
  reader: BitReader<'a>,
  /// Boxed to keep several kilobytes of coder state off the stack of every caller.
  tree: Box<DynamicHuffmanTree>,
  history: Box<RingBuffer>,
}

impl<'a> LzhufDecoder<'a> {
  pub(crate) fn new(stream: &'a [u8]) -> Self {
    Self {
      reader: BitReader::new(stream),
      tree: Box::new(DynamicHuffmanTree::new()),
      history: Box::default(),
    }
  }

  /// Decode exactly `target.len()` bytes.
  pub(crate) fn decode_into(&mut self, target: &mut [u8]) -> XrfResult<()> {
    let length: usize = target.len();
    let mut position: usize = 0;

    while position < length {
      let code: u16 = self.tree.read_code(&mut self.reader)?;

      if code < MATCH_CODE_BASE {
        let byte: u8 = code as u8;

        self.history.push(byte);
        target[position] = byte;
        position += 1;

        continue;
      }

      // Codes above the literal range carry a match length; the shortest match encoded is one longer
      // than the threshold, since anything at or below it is cheaper to emit as literals.
      let match_length: usize = usize::from(code - MATCH_CODE_BASE) + MATCH_LENGTH_THRESHOLD + 1;
      let distance: usize = usize::from(self.read_match_distance()?);

      for _ in 0..match_length {
        if position == length {
          break;
        }

        // Reading and pushing one byte at a time is what makes an overlapping match - a run that
        // reaches into bytes this same match produced - decode the way the encoder intended.
        let byte: u8 = self.history.byte_at_distance(distance);

        self.history.push(byte);
        target[position] = byte;
        position += 1;
      }
    }

    Ok(())
  }

  /// Read a match distance: nine leading bits select a bucket, which names the remaining bits to read.
  fn read_match_distance(&mut self) -> XrfResult<u16> {
    let leading: u16 = self.reader.read_bits(9)?;
    let (distance, remaining_bits) = split_match_distance(leading);

    Ok(distance | self.reader.read_bits(remaining_bits)?)
  }
}

/// Split the nine leading bits of a distance into its known high bits and the count still to be read.
///
/// The buckets encode near distances in fewer bits than far ones. Ranges are matched on the top four
/// bits, so the low bits of `leading` pass through into the result untouched.
fn split_match_distance(leading: u16) -> (u16, u32) {
  match leading & 0b1_1110_0000 {
    0b0_0000_0000..=0b0_0011_1111 => (leading, 0),
    0b0_0100_0000..=0b0_1001_1111 => ((leading - 0b0_0010_0000) << 1, 1),
    0b0_1010_0000..=0b1_0001_1111 => ((leading - 0b0_0110_0000) << 2, 2),
    0b1_0010_0000..=0b1_0111_1111 => ((leading - 0b0_1100_0000) << 3, 3),
    0b1_1000_0000..=0b1_1101_1111 => ((leading - 0b1_0010_0000) << 4, 4),
    // `leading` never exceeds nine bits, so this is the final bucket rather than an open end.
    _ => ((leading - 0b1_1000_0000) << 5, 5),
  }
}

#[cfg(test)]
mod tests {
  use super::split_match_distance;
  use crate::lzhuf_constants::RING_BUFFER_SIZE;

  #[test]
  fn spends_fewer_bits_on_near_distances() {
    assert_eq!(split_match_distance(0b0_0000_0000), (0, 0));
    assert_eq!(split_match_distance(0b0_0011_1111), (0b11_1111, 0));
    assert_eq!(split_match_distance(0b0_1001_1111), (0b1111_1110, 1));
    assert_eq!(split_match_distance(0b1_1111_1111), (0b1111_1110_0000, 5));
  }

  #[test]
  fn keeps_every_bucket_inside_the_history_window() {
    for leading in 0..0b10_0000_0000u16 {
      let (distance, remaining_bits) = split_match_distance(leading);
      let widest: usize = usize::from(distance) | ((1usize << remaining_bits) - 1);

      assert!(
        widest < RING_BUFFER_SIZE,
        "distance {widest} from leading bits {leading:#011b} escapes the window"
      );
    }
  }
}
