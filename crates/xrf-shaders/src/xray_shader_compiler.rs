use xrf_error::{XrfError, XrfResult};

use crate::{ShaderRenderer, XRayShader};

/// Compiles a fully resolved X-Ray shader source tree for one renderer.
pub trait XRayShaderCompiler {
  fn compile(&self, shader: &XRayShader, renderer: ShaderRenderer) -> XrfResult;
}

/// Placeholder until a renderer-specific compiler backend is implemented.
pub struct XRayShaderPlaceholderCompiler;

impl XRayShaderCompiler for XRayShaderPlaceholderCompiler {
  fn compile(&self, _shader: &XRayShader, renderer: ShaderRenderer) -> XrfResult {
    Err(XrfError::new_not_implemented_error(format!(
      "{} shader compilation is not implemented",
      renderer.display_name()
    )))
  }
}
