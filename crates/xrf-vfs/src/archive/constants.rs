/// High bit of a chunk id, set when the chunk's payload is compressed.
///
/// `CFS_CompressMark` in `xray-16/src/xrCore/FS.h`.
pub const CHUNK_ID_COMPRESSED_MASK: u32 = 1 << 31;
/// The chunk id itself, with the compression flag masked off.
pub const CHUNK_ID_MASK: u32 = !(1 << 31);

/// Chunk ids carrying the entry name table, of which a volume has one.
///
/// `1` is the engine's, read by `CLocatorAPI::LoadArchive`. `0x86` appears in volumes this tooling has to open but in no
/// reference engine tree, so its provenance is unverified — accepted because dropping it would make those volumes
/// unreadable, not because the format documents it.
pub const CHUNK_ID_FILE_DESCRIPTORS: [u32; 2] = [1, 0x86];

/// Chunk ids carrying the `[header]` metadata that names the volume's entry point.
///
/// `666` is `CFS_HeaderChunkID` in `xray-16/src/xrCore/FS.h`, which `CLocatorAPI::ProcessArchive` opens to read
/// `[header] entry_point`. `1337` is unverified in the same way as `0x86` above.
pub const CHUNK_ID_METADATA: [u32; 2] = [666, 1337];

/// Upper bound on an entry name, matching the fixed buffer a volume's name table is read through.
///
/// A header declaring a longer name is rejected rather than truncated: a truncated name is a different asset, so it
/// would shadow or miss silently.
pub const MAXIMUM_ENTRY_NAME_SIZE: usize = 520;
