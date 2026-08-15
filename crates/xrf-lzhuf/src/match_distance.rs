//! Variable-width coding of match distances, shared by both directions of the codec.
//!
//! A distance is coded as nine leading bits that select a bucket, followed by the bits the bucket still
//! needs. Near distances land in buckets that spend fewer total bits than far ones, so a nine bit read is
//! always enough to know how much more to read. [`split_match_distance`] and [`join_match_distance`] are
//! inverses, and the test below holds them to that over every distance the window can express.

/// Leading bits every distance spends before its bucket-specific remainder.
pub(crate) const LEADING_BIT_COUNT: u32 = 9;

/// Split the nine leading bits into the known high bits of a distance and the count still to be read.
///
/// Ranges are matched on the top four bits, so the low bits of `leading` pass through untouched.
pub(crate) fn split_match_distance(leading: u16) -> (u16, u32) {
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

/// Split a distance into its nine leading bits and the remainder bits that follow them.
///
/// Returns `(leading, remaining_bit_count, remaining)`. The caller writes `leading` in
/// [`LEADING_BIT_COUNT`] bits, then `remaining` in `remaining_bit_count` bits.
pub(crate) fn join_match_distance(distance: u16) -> (u16, u32, u16) {
  match distance {
    0..=63 => (distance, 0, 0),
    64..=255 => ((distance >> 1) + 0b0_0010_0000, 1, distance & 0b1),
    256..=767 => ((distance >> 2) + 0b0_0110_0000, 2, distance & 0b11),
    768..=1535 => ((distance >> 3) + 0b0_1100_0000, 3, distance & 0b111),
    1536..=3071 => ((distance >> 4) + 0b1_0010_0000, 4, distance & 0b1111),
    _ => ((distance >> 5) + 0b1_1000_0000, 5, distance & 0b1_1111),
  }
}

#[cfg(test)]
mod tests {
  use super::{LEADING_BIT_COUNT, join_match_distance, split_match_distance};
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
    for leading in 0..(1u16 << LEADING_BIT_COUNT) {
      let (distance, remaining_bits) = split_match_distance(leading);
      let widest: usize = usize::from(distance) | ((1usize << remaining_bits) - 1);

      assert!(
        widest < RING_BUFFER_SIZE,
        "distance {widest} from leading bits {leading:#011b} escapes the window"
      );
    }
  }

  /// The property the encoder depends on: whatever it writes, the decoder reads back unchanged.
  #[test]
  fn joining_and_splitting_round_trips_every_distance() {
    for distance in 0..RING_BUFFER_SIZE as u16 {
      let (leading, remaining_bits, remaining) = join_match_distance(distance);

      assert!(leading < (1 << LEADING_BIT_COUNT), "leading bits fit their field");
      assert!(remaining < (1 << remaining_bits), "remainder fits its field");

      let (high, expected_bits) = split_match_distance(leading);

      assert_eq!(
        expected_bits, remaining_bits,
        "distance {distance} disagrees on bit count"
      );
      assert_eq!(high | remaining, distance, "distance {distance} does not round trip");
    }
  }
}
