use crate::file::error::{LtxError, LtxParseError};
use crate::file::parser::LtxParser;
use crate::file::types::LtxIncludes;
use crate::{Ltx, ParseOptions, WriteOptions};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

impl Ltx {
  /// Load from a string.
  pub fn load_from_str(buf: &str) -> Result<Ltx, LtxParseError> {
    Ltx::load_from_str_opt(buf, ParseOptions::default())
  }

  /// Load from a string with options.
  pub fn load_from_str_opt(buf: &str, options: ParseOptions) -> Result<Ltx, LtxParseError> {
    LtxParser::new(buf.chars(), options).parse()
  }

  /// Load from a reader.
  pub fn read_from<R: Read>(reader: &mut R) -> Result<Ltx, LtxError> {
    Ltx::read_from_opt(reader, ParseOptions::default())
  }

  /// Load from a reader, but do not interpret '\' as an escape character.
  pub fn read_from_noescape<R: Read>(reader: &mut R) -> Result<Ltx, LtxError> {
    Ltx::read_from_opt(
      reader,
      ParseOptions {
        enabled_escape: false,
        ..ParseOptions::default()
      },
    )
  }

  /// Load from a reader with options.
  pub fn read_from_opt<R: Read>(reader: &mut R, options: ParseOptions) -> Result<Ltx, LtxError> {
    let mut data: String = String::new();

    reader.read_to_string(&mut data).map_err(LtxError::Io)?;

    match LtxParser::new(data.chars(), options).parse() {
      Err(e) => Err(LtxError::Parse(e)),
      Ok(success) => Ok(success),
    }
  }

  /// Load from a file.
  pub fn load_from_file<P: AsRef<Path>>(filename: P) -> Result<Ltx, LtxError> {
    Ltx::load_from_file_opt(filename, ParseOptions::default())
  }

  /// Load from a file, but do not interpret '\' as an escape character.
  pub fn load_from_file_noescape<P: AsRef<Path>>(filename: P) -> Result<Ltx, LtxError> {
    Ltx::load_from_file_opt(
      filename,
      ParseOptions {
        enabled_escape: false,
        ..ParseOptions::default()
      },
    )
  }

  /// Load from a file with options.
  pub fn load_from_file_full_inherited_opt<P: AsRef<Path>>(
    filename: P,
    options: ParseOptions,
  ) -> Result<Ltx, LtxError> {
    Ltx::load_from_file_opt(filename, options.clone())?
      .into_included_opt(options)?
      .into_inherited()
  }

  /// Load from a file with options.
  pub fn load_from_file_full_opt<P: AsRef<Path>>(
    filename: P,
    options: ParseOptions,
  ) -> Result<Ltx, LtxError> {
    Ltx::load_from_file_opt(filename, options.clone())?.into_included_opt(options)
  }

  /// Load from a file with options.
  pub fn load_from_file_opt<P: AsRef<Path>>(
    filename: P,
    options: ParseOptions,
  ) -> Result<Ltx, LtxError> {
    let mut reader: File = match File::open(filename.as_ref()) {
      Ok(file) => file,
      Err(error) => {
        return Err(LtxError::Io(error));
      }
    };

    match Ltx::read_from_opt(&mut reader, options) {
      Ok(mut ltx) => {
        ltx.path = Some(PathBuf::from(filename.as_ref()));
        ltx.directory = filename.as_ref().parent().map(PathBuf::from);

        Ok(ltx)
      }
      Err(error) => Err(error),
    }
  }
}

impl Ltx {
  /// Load include statements from a string.
  pub fn read_includes_from_str(buf: &str) -> Result<LtxIncludes, LtxParseError> {
    Ltx::read_includes_from_str_opt(buf, ParseOptions::default())
  }

  /// Load include statements from a string with options.
  pub fn read_includes_from_str_opt(
    buf: &str,
    options: ParseOptions,
  ) -> Result<LtxIncludes, LtxParseError> {
    LtxParser::new(buf.chars(), options).parse_includes()
  }

  /// Load include statements from a reader.
  pub fn read_includes_from<R: Read>(reader: &mut R) -> Result<LtxIncludes, LtxError> {
    Ltx::read_includes_from_opt(reader, ParseOptions::default())
  }

  /// Load include statements from a reader with options.
  pub fn read_includes_from_opt<R: Read>(
    reader: &mut R,
    options: ParseOptions,
  ) -> Result<LtxIncludes, LtxError> {
    let mut data: String = String::new();

    reader.read_to_string(&mut data).map_err(LtxError::Io)?;

    match LtxParser::new(data.chars(), options).parse_includes() {
      Err(error) => Err(LtxError::Parse(error)),
      Ok(success) => Ok(success),
    }
  }

  /// Load include statements from a file.
  pub fn read_includes_from_file<P: AsRef<Path>>(filename: P) -> Result<LtxIncludes, LtxError> {
    Ltx::read_includes_from_file_opt(filename, ParseOptions::default())
  }

  /// Load include statements from a file with options.
  pub fn read_includes_from_file_opt<P: AsRef<Path>>(
    filename: P,
    options: ParseOptions,
  ) -> Result<LtxIncludes, LtxError> {
    let mut reader: File = match File::open(filename.as_ref()) {
      Ok(file) => file,
      Err(error) => {
        return Err(LtxError::Io(error));
      }
    };

    Ltx::read_includes_from_opt(&mut reader, options)
  }
}

impl Ltx {
  /// Load from a string with options.
  pub fn format_from_str_opt(
    buf: &str,
    parse_options: ParseOptions,
    write_options: WriteOptions,
  ) -> Result<String, LtxParseError> {
    LtxParser::new(buf.chars(), parse_options).parse_into_formatted_opt(write_options)
  }

  /// Load from a string.
  pub fn format_from_str(buf: &str) -> Result<String, LtxParseError> {
    Ltx::format_from_str_opt(buf, ParseOptions::default(), WriteOptions::default())
  }

  /// Load from a reader with options.
  pub fn format_from_opt<R: Read>(
    reader: &mut R,
    parse_options: ParseOptions,
    write_options: WriteOptions,
  ) -> Result<String, LtxError> {
    let mut data: String = String::new();

    reader.read_to_string(&mut data).map_err(LtxError::Io)?;

    match LtxParser::new(data.chars(), parse_options).parse_into_formatted_opt(write_options) {
      Err(e) => Err(LtxError::Parse(e)),
      Ok(success) => Ok(success),
    }
  }

  /// Load from a file with options
  pub fn format_from_file_opt<P: AsRef<Path>>(
    filename: P,
    parse_options: ParseOptions,
    write_options: WriteOptions,
  ) -> Result<String, LtxError> {
    let mut reader: File = match File::open(filename.as_ref()) {
      Ok(file) => file,
      Err(error) => {
        return Err(LtxError::Io(error));
      }
    };

    Ltx::format_from_opt(&mut reader, parse_options, write_options)
  }
}

#[cfg(test)]
mod test {
  use crate::file::configuration::line_separator::LineSeparator;
  use crate::file::types::LtxIncludes;
  use crate::test::file::read_file_as_string;
  use crate::test::utils::{get_absolute_test_file_path, get_absolute_test_resource_as_file};
  use crate::{EscapePolicy, Ltx, ParseOptions, WriteOptions};
  use std::env::temp_dir;
  use std::fs::File;
  use std::io::Write;
  use std::path::PathBuf;

  #[test]
  fn load_from_file() {
    let file_name = temp_dir().join("rust_ini_load_from_file");
    let file_content = b"[test]Key=Value\n";

    {
      let mut file = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ltx: Ltx = Ltx::load_from_file(&file_name).unwrap();
    assert_eq!(ltx.get_from("test", "Key"), Some("Value"));
  }

  #[test]
  fn format_from_file_one() {
    let formatted: String = Ltx::format_from_file_opt(
      get_absolute_test_file_path(file!(), "not_formatted_1.ltx"),
      ParseOptions {
        enabled_escape: false,
        enabled_quote: false,
      },
      WriteOptions {
        escape_policy: EscapePolicy::Nothing,
        line_separator: LineSeparator::SystemDefault,
        ..Default::default()
      },
    )
    .unwrap();

    let expected: String = read_file_as_string(
      &mut get_absolute_test_resource_as_file(file!(), "formatted_1.ltx").unwrap(),
    )
    .unwrap();

    assert_eq!(formatted, expected);
  }

  #[test]
  fn format_from_file_two() {
    let formatted: String = Ltx::format_from_file_opt(
      get_absolute_test_file_path(file!(), "not_formatted_2.ltx"),
      ParseOptions {
        enabled_escape: false,
        enabled_quote: false,
      },
      WriteOptions {
        escape_policy: EscapePolicy::Nothing,
        line_separator: LineSeparator::SystemDefault,
        ..Default::default()
      },
    )
    .unwrap();

    let expected: String = read_file_as_string(
      &mut get_absolute_test_resource_as_file(file!(), "formatted_2.ltx").unwrap(),
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

    let includes: LtxIncludes = Ltx::read_includes_from_file(&file_name).unwrap();
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

    let includes: LtxIncludes = Ltx::read_includes_from_file(&file_name).unwrap();
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
