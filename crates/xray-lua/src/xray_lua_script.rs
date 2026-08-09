use std::path::{Path, PathBuf};

use full_moon::{LuaVersion, ast::Ast, parse_fallible};
use xray_error::{XRayError, XRayResult};

use crate::XRayLuaMethodCall;
use crate::lua_method_call_collector::LuaMethodCallCollector;

/// A parsed LuaJIT script with normalized method calls.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XRayLuaScript {
  method_calls: Vec<XRayLuaMethodCall>,
  path: PathBuf,
}

impl XRayLuaScript {
  /// Parse LuaJIT source and retain the source path in any diagnostics.
  pub fn parse<P>(path: P, source: &str) -> XRayResult<Self>
  where
    P: AsRef<Path>,
  {
    let path: &Path = path.as_ref();
    let ast: Ast = Self::parse_ast(path, source)?;

    Ok(Self {
      method_calls: LuaMethodCallCollector::collect(&ast),
      path: path.to_path_buf(),
    })
  }

  pub fn method_calls(&self, receiver: &str, method: &str) -> Vec<&XRayLuaMethodCall> {
    self
      .method_calls
      .iter()
      .filter(|call| call.receiver() == receiver && call.method() == method)
      .collect()
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  fn parse_ast(path: &Path, source: &str) -> XRayResult<Ast> {
    parse_fallible(source, LuaVersion::luajit())
      .into_result()
      .map_err(|errors| {
        XRayError::new_verify_error(format!(
          "Failed to check LuaJIT script file: {}, errors: {}",
          path.display(),
          errors
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        ))
      })
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use xray_error::XRayResult;

  use super::XRayLuaScript;

  #[test]
  fn collects_literal_and_dynamic_method_calls() -> XRayResult {
    let script: XRayLuaScript = XRayLuaScript::parse(
      Path::new("script.s"),
      r#"
shader:begin("vertex", "pixel")
shader:begin(dynamic_vertex, "dynamic_pixel")
other:begin("ignored_vertex", "ignored_pixel")
"#,
    )?;
    let shader_begins: Vec<&_> = script.method_calls("shader", "begin");

    assert_eq!(script.path(), Path::new("script.s"));
    assert_eq!(shader_begins.len(), 2);
    assert_eq!(
      shader_begins[0].literal_string_arguments(),
      Some([String::from("vertex"), String::from("pixel")].as_slice())
    );
    assert_eq!(shader_begins[1].literal_string_arguments(), None);

    Ok(())
  }
}
