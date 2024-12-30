use crate::file::parser::LtxParser;
use crate::file::types::LtxIncluded;
use crate::{Ltx, LtxError, LtxParseError};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

impl Ltx {
  /// Read LTX from a string.
  pub fn read_from_str(buf: &str) -> Result<Ltx, LtxParseError> {
    LtxParser::new(buf.chars()).parse()
  }

  /// Read LTX from a file as full parsed file, inject included files.
  pub fn read_from_file_included<P: AsRef<Path>>(filename: P) -> Result<Ltx, LtxError> {
    Ltx::read_from_path(filename)?.into_included()
  }

  /// Read LTX from a file, inject all includes and unwrap inherited sections.
  pub fn load_from_file_full<P: AsRef<Path>>(filename: P) -> Result<Ltx, LtxError> {
    Ltx::read_from_path(filename)?
      .into_included()?
      .into_inherited()
  }

  /// Read from a file as generic ini with LTX descriptor filled.
  pub fn read_from_path<P: AsRef<Path>>(filename: P) -> Result<Ltx, LtxError> {
    let mut reader: File = match File::open(filename.as_ref()) {
      Ok(file) => file,
      Err(error) => {
        return Err(LtxError::Io(error));
      }
    };

    match Ltx::read_from(&mut reader) {
      Ok(mut ltx) => {
        ltx.path = Some(PathBuf::from(filename.as_ref()));
        ltx.directory = filename.as_ref().parent().map(PathBuf::from);

        Ok(ltx)
      }
      Err(error) => Err(error),
    }
  }

  /// Read from a reader as generic ini with LTX descriptor filled.
  pub fn read_from<R: Read>(reader: &mut R) -> Result<Ltx, LtxError> {
    let mut data: String = String::new();

    reader.read_to_string(&mut data).map_err(LtxError::Io)?;

    match LtxParser::new(data.chars()).parse() {
      Err(error) => Err(LtxError::Parse(error)),
      Ok(success) => Ok(success),
    }
  }
}

impl Ltx {
  /// Load include statements from a string.
  pub fn read_included_from_str(buf: &str) -> Result<LtxIncluded, LtxParseError> {
    LtxParser::new(buf.chars()).parse_includes()
  }

  /// Load include statements from a file with options.
  pub fn read_included_from_file<P: AsRef<Path>>(filename: P) -> Result<LtxIncluded, LtxError> {
    let mut reader: File = match File::open(filename.as_ref()) {
      Ok(file) => file,
      Err(error) => {
        return Err(LtxError::Io(error));
      }
    };

    Ltx::read_included_from(&mut reader)
  }

  /// Load include statements from a reader.
  pub fn read_included_from<R: Read>(reader: &mut R) -> Result<LtxIncluded, LtxError> {
    let mut data: String = String::new();

    reader.read_to_string(&mut data).map_err(LtxError::Io)?;

    match LtxParser::new(data.chars()).parse_includes() {
      Err(error) => Err(LtxError::Parse(error)),
      Ok(success) => Ok(success),
    }
  }
}

impl Ltx {
  /// Load formatted LTX as string from string.
  pub fn format_from_str(buf: &str) -> Result<String, LtxParseError> {
    LtxParser::new(buf.chars()).parse_into_formatted()
  }

  /// Load formatted LTX as string from file.
  pub fn format_from_file<P: AsRef<Path>>(filename: P) -> Result<String, LtxError> {
    let mut reader: File = match File::open(filename.as_ref()) {
      Ok(file) => file,
      Err(error) => {
        return Err(LtxError::Io(error));
      }
    };

    Ltx::format_from(&mut reader)
  }

  /// Load formatted LTX as string from reader.
  pub fn format_from<R: Read>(reader: &mut R) -> Result<String, LtxError> {
    let mut data: String = String::new();

    reader.read_to_string(&mut data).map_err(LtxError::Io)?;

    match LtxParser::new(data.chars()).parse_into_formatted() {
      Err(e) => Err(LtxError::Parse(e)),
      Ok(success) => Ok(success),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::file::types::LtxIncluded;
  use crate::Ltx;
  use std::env::temp_dir;
  use std::fs::File;
  use std::io::Write;
  use std::path::PathBuf;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{get_absolute_test_file_path, get_absolute_test_resource_as_file};

  #[test]
  fn load_from_file() {
    let file_name = temp_dir().join("rust_ini_load_from_file");
    let file_content = b"[test]\nKey=Value\n";

    {
      let mut file: File = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ltx: Ltx = Ltx::read_from_path(&file_name).unwrap();
    assert_eq!(ltx.get_from("test", "Key"), Some("Value"));
  }

  #[test]
  fn format_from_file_one() {
    let formatted: String =
      Ltx::format_from_file(get_absolute_test_file_path(file!(), "not_formatted_1.ltx")).unwrap();

    let expected: String = read_file_as_string(
      &mut get_absolute_test_resource_as_file(file!(), "formatted_1.ltx").unwrap(),
    )
    .unwrap();

    assert_eq!(formatted, expected);
  }

  #[test]
  fn format_from_file_two() {
    let formatted: String =
      Ltx::format_from_file(get_absolute_test_file_path(file!(), "not_formatted_2.ltx")).unwrap();

    let expected: String = read_file_as_string(
      &mut get_absolute_test_resource_as_file(file!(), "formatted_2.ltx").unwrap(),
    )
    .unwrap();

    assert_eq!(formatted, expected);
  }

  #[test]
  fn format_from_file_three() {
    let formatted: String =
      Ltx::format_from_file(get_absolute_test_file_path(file!(), "not_formatted_3.ltx")).unwrap();

    let expected: String = read_file_as_string(
      &mut get_absolute_test_resource_as_file(file!(), "formatted_3.ltx").unwrap(),
    )
    .unwrap();

    assert_eq!(formatted, expected);
  }

  #[test]
  fn format_from_file_four() {
    let formatted: String =
      Ltx::format_from_file(get_absolute_test_file_path(file!(), "not_formatted_4.ltx")).unwrap();

    let expected: String = read_file_as_string(
      &mut get_absolute_test_resource_as_file(file!(), "formatted_4.ltx").unwrap(),
    )
    .unwrap();

    assert_eq!(formatted, expected);
  }

  #[test]
  fn load_no_includes_from_file() {
    let file_name: PathBuf = temp_dir().join("rust_ini_load_no_includes");
    let file_content = b"[test]Key=Value\n";

    {
      let mut file: File = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let includes: LtxIncluded = Ltx::read_included_from_file(&file_name).unwrap();
    assert_eq!(includes, Vec::<String>::new());
  }

  #[test]
  fn load_few_includes_from_file() {
    let file_name: PathBuf = temp_dir().join("rust_ini_load_from_file_without_bom");
    let file_content = b"#include \"first.ltx\"\n;commented\n#include \"second.ltx\"";

    {
      let mut file: File = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let includes: LtxIncluded = Ltx::read_included_from_file(&file_name).unwrap();
    assert_eq!(includes, vec!("first.ltx", "second.ltx"));
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
