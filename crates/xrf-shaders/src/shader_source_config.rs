use std::path::Path;

/// File extensions the engine treats as renderer shader sources.
pub const SHADER_SOURCE_FILE_EXTENSIONS: [&str; 7] = ["h", "vs", "ps", "cs", "hs", "ds", "gs"];

/// Lua renderer-definition script extension.
pub const SHADER_SCRIPT_FILE_EXTENSION: &str = "s";

/// Whether a path has a shader source extension recognized by the engine.
pub fn is_shader_source_path(path: &Path) -> bool {
  path
    .extension()
    .and_then(|value| value.to_str())
    .is_some_and(|extension| {
      SHADER_SOURCE_FILE_EXTENSIONS
        .iter()
        .any(|known_extension| extension.eq_ignore_ascii_case(known_extension))
    })
}
