/// Extensions that contain plain text and not binary data.
pub const ALLOWED_PROJECT_READ_EXTENSIONS: &[&str] = &["ltx", "script", "ps", "ds", "h", "hs", "s", "vs", "cmd", "xml"];

pub const ALLOWED_PROJECT_READ_SIZE: u32 = 10 * 1024 * 1024; // 10 MBytes
