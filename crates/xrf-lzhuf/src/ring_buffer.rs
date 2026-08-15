use crate::lzhuf_constants::{RING_BUFFER_FILLER, RING_BUFFER_SIZE};

/// Mask that wraps any index into the window, which is why [`RING_BUFFER_SIZE`] must be a power of two.
const INDEX_MASK: usize = RING_BUFFER_SIZE - 1;

/// The sliding window of already-coded bytes that match commands refer back into.
///
/// Pre-filled with [`RING_BUFFER_FILLER`] like `LzHuf.cpp` does, so a stream may legally reference history
/// that was never written. Only distances relative to the write cursor are ever used, so the absolute
/// cursor the encoder started from does not matter.
#[derive(Clone)]
pub(crate) struct RingBuffer {
  buffer: [u8; RING_BUFFER_SIZE],
  cursor: usize,
}

impl Default for RingBuffer {
  fn default() -> Self {
    Self {
      buffer: [RING_BUFFER_FILLER; RING_BUFFER_SIZE],
      cursor: 0,
    }
  }
}

impl RingBuffer {
  /// Append one byte, evicting the oldest.
  pub(crate) fn push(&mut self, byte: u8) {
    self.buffer[self.cursor] = byte;
    self.cursor = (self.cursor + 1) & INDEX_MASK;
  }

  /// Read a byte back from the write cursor, where distance 0 is the most recently pushed byte.
  ///
  /// Distances wrap, so an over-long distance names an older byte instead of failing.
  pub(crate) fn byte_at_distance(&self, distance: usize) -> u8 {
    self.buffer[(self.cursor + RING_BUFFER_SIZE - (distance & INDEX_MASK) - 1) & INDEX_MASK]
  }
}

#[cfg(test)]
mod tests {
  use super::{INDEX_MASK, RingBuffer};
  use crate::lzhuf_constants::{RING_BUFFER_FILLER, RING_BUFFER_SIZE};

  #[test]
  fn starts_filled_so_unwritten_history_is_defined() {
    let buffer: RingBuffer = RingBuffer::default();

    for distance in 0..RING_BUFFER_SIZE {
      assert_eq!(buffer.byte_at_distance(distance), RING_BUFFER_FILLER);
    }
  }

  #[test]
  fn reads_back_from_the_write_cursor() {
    let mut buffer: RingBuffer = RingBuffer::default();

    for byte in *b"xray" {
      buffer.push(byte);
    }

    assert_eq!(buffer.byte_at_distance(0), b'y');
    assert_eq!(buffer.byte_at_distance(1), b'a');
    assert_eq!(buffer.byte_at_distance(2), b'r');
    assert_eq!(buffer.byte_at_distance(3), b'x');
    assert_eq!(buffer.byte_at_distance(4), RING_BUFFER_FILLER);
  }

  #[test]
  fn wraps_around_the_window() {
    let mut buffer: RingBuffer = RingBuffer::default();

    for index in 0..RING_BUFFER_SIZE + 8 {
      buffer.push(index as u8);
    }

    assert_eq!(buffer.byte_at_distance(0), (RING_BUFFER_SIZE + 7) as u8);
    assert_eq!(buffer.byte_at_distance(INDEX_MASK), (RING_BUFFER_SIZE + 8) as u8);
  }
}
