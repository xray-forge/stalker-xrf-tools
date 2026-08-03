use crate::XRayShaderPass;
use std::path::{Path, PathBuf};
use xray_error::XRayResult;
use xray_lua::XRayLuaScript;

/// An X-Ray renderer shader script and the literal passes it declares.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XRayShaderScript {
  passes: Vec<XRayShaderPass>,
  path: PathBuf,
}

impl XRayShaderScript {
  /// Parse an X-Ray shader script and collect literal `shader:begin` calls.
  pub fn parse<P>(path: P, source: &str) -> XRayResult<Self>
  where
    P: AsRef<Path>,
  {
    let path: &Path = path.as_ref();
    let lua_script: XRayLuaScript = XRayLuaScript::parse(path, source)?;
    let passes: Vec<XRayShaderPass> = lua_script
      .method_calls("shader", "begin")
      .into_iter()
      .filter_map(|method_call| {
        let [vertex_shader, pixel_shader] = method_call.literal_string_arguments()? else {
          return None;
        };

        Some(XRayShaderPass::from_literal_shader_begin(
          method_call.line_number(),
          vertex_shader.clone(),
          pixel_shader.clone(),
        ))
      })
      .collect();

    Ok(Self {
      passes,
      path: path.to_path_buf(),
    })
  }

  pub fn passes(&self) -> &[XRayShaderPass] {
    &self.passes
  }

  pub fn path(&self) -> &Path {
    &self.path
  }
}

#[cfg(test)]
mod tests {
  use super::XRayShaderScript;
  use std::path::Path;
  use xray_error::{XRayError, XRayResult};

  #[test]
  fn collects_literal_shader_passes() -> XRayResult {
    let script: XRayShaderScript = XRayShaderScript::parse(
      Path::new("shaders/r3/example.s"),
      r#"
function normal(shader)
  shader:begin("vertex", "pixel"):sorting(1, false)
  shader:begin(dynamic_vertex, "dynamic_pixel")
  other:begin("ignored_vertex", "ignored_pixel")
  -- shader:begin("commented_vertex", "commented_pixel")
end
"#,
    )?;

    assert_eq!(script.path(), Path::new("shaders/r3/example.s"));
    assert_eq!(script.passes().len(), 1);
    assert_eq!(script.passes()[0].vertex_shader(), "vertex");
    assert_eq!(script.passes()[0].pixel_shader(), "pixel");

    Ok(())
  }

  #[test]
  fn reports_luajit_syntax_errors() {
    let result = XRayShaderScript::parse(Path::new("invalid.s"), "function normal(");

    assert!(matches!(result, Err(XRayError::Verify { .. })));
  }
}
