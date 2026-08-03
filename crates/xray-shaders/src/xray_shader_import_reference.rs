use std::path::Path;
use xray_error::{XRayError, XRayResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XRayShaderImportReference {
  line_number: usize,
  path: String,
}

impl XRayShaderImportReference {
  pub fn parse_all(path: &Path, source: &[u8]) -> XRayResult<Vec<Self>> {
    let mut imports: Vec<Self> = Vec::new();

    for (index, line) in String::from_utf8_lossy(source).lines().enumerate() {
      let line_number: usize = index + 1;
      let line: &str = line.trim_start();

      let Some(include) = line.strip_prefix("#include") else {
        continue;
      };

      let include: &str = include.trim_start();

      let Some(include) = include.strip_prefix('"') else {
        return Err(XRayError::new_invalid_error(format!(
          "Shader {} has malformed #include on line {line_number}: expected a quoted import path",
          path.display()
        )));
      };

      let Some(end) = include.find('"') else {
        return Err(XRayError::new_invalid_error(format!(
          "Shader {} has malformed #include on line {line_number}: expected a closing quote",
          path.display()
        )));
      };

      imports.push(Self {
        line_number,
        path: include[..end].to_string(),
      });
    }

    Ok(imports)
  }

  pub fn line_number(&self) -> usize {
    self.line_number
  }

  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn into_path(self) -> String {
    self.path
  }
}
