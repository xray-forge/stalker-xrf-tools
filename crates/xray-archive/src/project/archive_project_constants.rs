/// Extensions that contain plain text and not binary data.
pub const ALLOWED_PROJECT_READ_EXTENSIONS: &[&str] = &["ltx", "script", "ps", "ds", "h", "hs", "s", "vs", "cmd", "xml"];

pub const ALLOWED_PROJECT_READ_SIZE: u32 = 10 * 1024 * 1024; // 10 MBytes

/// Extensions the backend can decode into a picture rather than read as text.
pub const ALLOWED_PROJECT_IMAGE_EXTENSIONS: &[&str] = &["dds"];

/// Upper bound on an image entry, guarding against holding a very large texture in memory to decode.
pub const ALLOWED_PROJECT_IMAGE_SIZE: u32 = 32 * 1024 * 1024; // 32 MBytes
