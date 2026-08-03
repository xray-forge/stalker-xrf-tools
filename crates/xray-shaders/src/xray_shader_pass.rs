/// A literal vertex and pixel shader pair selected by `shader:begin`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XRayShaderPass {
  line_number: usize,
  pixel_shader: String,
  vertex_shader: String,
}

impl XRayShaderPass {
  pub fn from_literal_shader_begin(
    line_number: usize,
    vertex_shader: String,
    pixel_shader: String,
  ) -> Self {
    Self {
      line_number,
      pixel_shader,
      vertex_shader,
    }
  }

  pub fn line_number(&self) -> usize {
    self.line_number
  }

  pub fn pixel_shader(&self) -> &str {
    &self.pixel_shader
  }

  pub fn vertex_shader(&self) -> &str {
    &self.vertex_shader
  }
}
