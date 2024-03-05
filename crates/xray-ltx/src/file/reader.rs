use crate::file::error::{LtxError, LtxParseError};
use crate::file::parser::LtxParser;
use crate::{Ltx, ParseOptions};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

impl Ltx {
  /// Load from a string
  pub fn load_from_str(buf: &str) -> Result<Ltx, LtxParseError> {
    Ltx::load_from_str_opt(buf, ParseOptions::default())
  }

  /// Load from a string, but do not interpret '\' as an escape character
  pub fn load_from_str_noescape(buf: &str) -> Result<Ltx, LtxParseError> {
    Ltx::load_from_str_opt(
      buf,
      ParseOptions {
        enabled_escape: false,
        ..ParseOptions::default()
      },
    )
  }

  /// Load from a string with options
  pub fn load_from_str_opt(buf: &str, opt: ParseOptions) -> Result<Ltx, LtxParseError> {
    let mut parser = LtxParser::new(buf.chars(), opt);
    parser.parse()
  }

  /// Load from a reader
  pub fn read_from<R: Read>(reader: &mut R) -> Result<Ltx, LtxError> {
    Ltx::read_from_opt(reader, ParseOptions::default())
  }

  /// Load from a reader, but do not interpret '\' as an escape character
  pub fn read_from_noescape<R: Read>(reader: &mut R) -> Result<Ltx, LtxError> {
    Ltx::read_from_opt(
      reader,
      ParseOptions {
        enabled_escape: false,
        ..ParseOptions::default()
      },
    )
  }

  /// Load from a reader with options
  pub fn read_from_opt<R: Read>(reader: &mut R, opt: ParseOptions) -> Result<Ltx, LtxError> {
    let mut s = String::new();
    reader.read_to_string(&mut s).map_err(LtxError::Io)?;
    let mut parser = LtxParser::new(s.chars(), opt);
    match parser.parse() {
      Err(e) => Err(LtxError::Parse(e)),
      Ok(success) => Ok(success),
    }
  }

  /// Load from a file
  pub fn load_from_file<P: AsRef<Path>>(filename: P) -> Result<Ltx, LtxError> {
    Ltx::load_from_file_opt(filename, ParseOptions::default())
  }

  /// Load from a file, but do not interpret '\' as an escape character
  pub fn load_from_file_noescape<P: AsRef<Path>>(filename: P) -> Result<Ltx, LtxError> {
    Ltx::load_from_file_opt(
      filename,
      ParseOptions {
        enabled_escape: false,
        ..ParseOptions::default()
      },
    )
  }

  /// Load from a file with options
  pub fn load_from_file_opt<P: AsRef<Path>>(
    filename: P,
    options: ParseOptions,
  ) -> Result<Ltx, LtxError> {
    let mut reader: File = match File::open(filename.as_ref()) {
      Err(error) => {
        return Err(LtxError::Io(error));
      }
      Ok(r) => r,
    };

    let mut with_bom: bool = false;

    // Check if file starts with a BOM marker
    // UTF-8: EF BB BF
    let mut bom = [0u8; 3];
    if reader.read_exact(&mut bom).is_ok() && &bom == b"\xEF\xBB\xBF" {
      with_bom = true;
    }

    // Reset file pointer
    if !with_bom {
      reader.seek(SeekFrom::Start(0))?;
    }

    Ltx::read_from_opt(&mut reader, options)
  }
}

#[cfg(test)]
mod test {
  use crate::Ltx;
  use std::env::temp_dir;
  use std::fs::File;
  use std::io::Write;

  #[test]
  fn load_from_file_with_bom() {
    let file_name = temp_dir().join("rust_ini_load_from_file_with_bom");
    let file_content = b"\xEF\xBB\xBF[test]Key=Value\n";

    {
      let mut file = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ltx: Ltx = Ltx::load_from_file(&file_name).unwrap();
    assert_eq!(ltx.get_from("test", "Key"), Some("Value"));
  }

  #[test]
  fn load_from_file_without_bom() {
    let file_name = temp_dir().join("rust_ini_load_from_file_without_bom");

    let file_content = b"[test]Key=Value\n";

    {
      let mut file = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ltx: Ltx = Ltx::load_from_file(&file_name).unwrap();
    assert_eq!(ltx.get_from("test", "Key"), Some("Value"));
  }

  #[test]
  fn invalid_codepoint() {
    use std::io::Cursor;

    let d = vec![
      10, 8, 68, 8, 61, 10, 126, 126, 61, 49, 10, 62, 8, 8, 61, 10, 91, 93, 93, 36, 91, 61, 10, 75,
      91, 10, 10, 10, 61, 92, 120, 68, 70, 70, 70, 70, 70, 126, 61, 10, 0, 0, 61, 10, 38, 46, 49,
      61, 0, 39, 0, 0, 46, 92, 120, 46, 36, 91, 91, 1, 0, 0, 16, 0, 0, 0, 0, 0, 0,
    ];
    let mut file = Cursor::new(d);
    assert!(Ltx::read_from(&mut file).is_err());
  }
}
