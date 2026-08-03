use crate::XRayLuaScript;
use std::path::Path;
use xray_error::XRayResult;

/// Verify LuaJIT source syntax.
pub fn verify_luajit_script(code: &str, path: &Path) -> XRayResult<()> {
  XRayLuaScript::parse(path, code).map(|_| ())
}

#[cfg(test)]
mod tests {
  use super::verify_luajit_script;
  use std::path::Path;

  #[test]
  fn accepts_luajit_goto_and_label() {
    let code: &str = r#"
local ____exports = {}

for index = 1, 3 do
  if index == 2 then
    goto __continue1
  end

  ::__continue1::
end

return ____exports
"#;

    assert!(verify_luajit_script(code, Path::new("generated.script")).is_ok());
  }

  #[test]
  fn reports_invalid_luajit_syntax_with_source_path() {
    let path: &Path = Path::new("invalid.script");
    let error: String = verify_luajit_script("local value =", path)
      .expect_err("Expected malformed LuaJIT script to fail")
      .to_string();

    assert!(error.contains("invalid.script"));
    assert!(error.contains("errors:"));
  }
}
