//! Tuning constants of the X-Ray LZHUF codec, named after `xrCore/LzHuf.cpp`.

/// Sliding history window, in bytes (`N`). Must stay a power of two: distances are masked, not compared.
pub(crate) const RING_BUFFER_SIZE: usize = 4096;

/// Longest match a single command can encode (`F`).
pub(crate) const MATCH_LENGTH_MAX: usize = 60;

/// Longest run still cheaper to emit as literals (`THRESHOLD`).
pub(crate) const MATCH_LENGTH_THRESHOLD: usize = 2;

/// First command code that means "copy from history" rather than "emit this byte".
pub(crate) const MATCH_CODE_BASE: u16 = 256;

/// Literal bytes plus every encodable match length (`N_CHAR`), which is the leaf count of the tree.
pub(crate) const LEAF_COUNT: usize = 256 - MATCH_LENGTH_THRESHOLD + MATCH_LENGTH_MAX;

/// Total node count of the adaptive tree (`T`): a full binary tree over [`LEAF_COUNT`] leaves.
pub(crate) const NODE_COUNT: usize = LEAF_COUNT * 2 - 1;

/// Root frequency that forces a tree rebuild (`MAX_FREQ`).
///
/// X-Ray halves the 32768 of standard LHA `-lh1-`, so an X-Ray stream and an LHA stream diverge once
/// 16384 symbols have been coded: every tree rebuild from that point lands on different code lengths.
/// A stock LHA decoder therefore decodes the head of an X-Ray stream correctly and turns the tail into
/// garbage. This single constant is the whole reason XRF cannot use an off-the-shelf LZH library.
pub(crate) const REBUILD_FREQUENCY_LIMIT: u16 = 0x4000;

/// Byte the history window is pre-filled with, so back-references into never-written history are defined.
pub(crate) const RING_BUFFER_FILLER: u8 = b' ';

/// Length of the little-endian `u32` decompressed-size header that precedes an X-Ray LZHUF stream.
pub(crate) const DECOMPRESSED_SIZE_HEADER_LENGTH: usize = 4;
