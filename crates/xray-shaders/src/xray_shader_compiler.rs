use xray_error::{XRayError, XRayResult};

use crate::{ShaderRenderer, XRayShader};

/// Compiles a fully resolved X-Ray shader source tree for one renderer.
pub trait XRayShaderCompiler {
  fn compile(&self, shader: &XRayShader, renderer: ShaderRenderer) -> XRayResult;
}

/// Placeholder until a renderer-specific compiler backend is implemented.
pub struct XRayShaderPlaceholderCompiler;

impl XRayShaderCompiler for XRayShaderPlaceholderCompiler {
  fn compile(&self, _shader: &XRayShader, renderer: ShaderRenderer) -> XRayResult {
    Err(XRayError::new_not_implemented_error(format!(
      "{} shader compilation is not implemented",
      renderer.display_name()
    )))
  }
}
