use xrf_error::XrfResult;

use crate::bit_reader::BitReader;
use crate::lzhuf_constants::{MATCH_CODE_BASE, MATCH_LENGTH_THRESHOLD};
use crate::match_distance::{LEADING_BIT_COUNT, split_match_distance};
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
    let leading: u16 = self.reader.read_bits(LEADING_BIT_COUNT)?;
    let (distance, remaining_bits) = split_match_distance(leading);

    Ok(distance | self.reader.read_bits(remaining_bits)?)
  }
}
