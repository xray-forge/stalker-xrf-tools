use crate::file::configuration::constants::ROOT_SECTION;
use crate::file::configuration::line_separator::{LineSeparator, DEFAULT_KV_SEPARATOR};
use crate::{Ltx, LtxResult};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

impl Ltx {
  /// Format single LTX file by provided path
  pub fn format_file<P: AsRef<Path>>(filename: P, write: bool) -> LtxResult<bool> {
    let formatted: String = Ltx::format_from_file(&filename)?;
    let existing: String =
      io::read_to_string(&mut OpenOptions::new().read(true).open(filename.as_ref())?)?;

    if existing == formatted {
      Ok(false)
    } else {
      if write {
        fs::write(&filename, formatted)?;
      }

      Ok(true)
    }
  }

  /// Write to a file
  pub fn write_to_path<P: AsRef<Path>>(&self, filename: P) -> LtxResult {
    self.write_to(
      &mut OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename.as_ref())?,
    )
  }

  /// Write to a writer with options
  pub fn write_to<W: Write>(&self, writer: &mut W) -> LtxResult {
    let mut firstline: bool = true;

    // Write include statements.
    if !self.includes.is_empty() {
      firstline = false;

      for include in &self.includes {
        write!(writer, "#include \"{}\"{}", include, LineSeparator::CRLF)?;
      }
    }

    for (section, props) in &self.sections {
      // If root section with data or generic section.
      if section != ROOT_SECTION || !props.data.is_empty() {
        if firstline {
          firstline = false;
        } else {
          // Write an empty line between sections
          writer.write_all(LineSeparator::CRLF.as_str().as_bytes())?;
        }
      }

      if section != ROOT_SECTION {
        let inherited: String = if props.inherited.is_empty() {
          String::new()
        } else {
          format!(":{}", props.inherited.join(", "))
        };

        write!(writer, "[{}]{}{}", section, inherited, LineSeparator::CRLF)?;
      }

      for (key, value) in props.iter() {
        write!(
          writer,
          "{}{}{}{}",
          key,
          DEFAULT_KV_SEPARATOR,
          value,
          LineSeparator::CRLF
        )?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::file::configuration::line_separator::DEFAULT_LINE_SEPARATOR;
  use crate::{Ltx, ROOT_SECTION};

  #[test]
  fn preserve_order_write() {
    let input = r"
x2 = n2
x1 = n2
x3 = n2
[s]
x2 = n2
xb = n2
a3 = n3
";
    let ltx: Ltx = Ltx::read_from_str(input).unwrap();
    let mut buf = vec![];
    ltx.write_to(&mut buf).unwrap();
    let mut new_data = Ltx::read_from_str(&String::from_utf8(buf).unwrap()).unwrap();

    let sec0 = new_data.root_section();
    let keys0: Vec<&str> = sec0.iter().map(|(k, _)| k).collect();
    assert_eq!(keys0, vec!["x2", "x1", "x3"]);

    let sec1 = new_data.section("s").unwrap();
    let keys1: Vec<&str> = sec1.iter().map(|(k, _)| k).collect();
    assert_eq!(keys1, vec!["x2", "xb", "a3"]);
  }

  #[test]
  fn write_new() {
    use std::str;

    let ltx: Ltx = Ltx::new();

    let mut buf = Vec::new();
    ltx.write_to(&mut buf).unwrap();

    assert_eq!("", str::from_utf8(&buf).unwrap());
  }

  #[test]
  fn write_line_separator() {
    use std::str;

    let mut ini = Ltx::new();
    ini
      .with_section("Section1")
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section("Section2")
      .set("Key1", "Value")
      .set("Key2", "Value");

    {
      let mut buf: Vec<u8> = Vec::new();
      ini.write_to(&mut buf).unwrap();

      assert_eq!(
        "[Section1]\r\nKey1 = Value\r\nKey2 = Value\r\n\r\n[Section2]\r\nKey1 = Value\r\nKey2 = Value\r\n",
        str::from_utf8(&buf).unwrap()
      );
    }
  }

  #[test]
  fn write_kv_separator() {
    use std::str;

    let mut ini = Ltx::new();

    ini
      .with_section(ROOT_SECTION)
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section("Section1")
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section("Section2")
      .set("Key1", "Value")
      .set("Key2", "Value");

    let mut buf: Vec<u8> = Vec::new();
    ini.write_to(&mut buf).unwrap();

    // Test different line endings in Windows and Unix
    if cfg!(windows) {
      assert_eq!(
        str::from_utf8(&buf).unwrap(),
        "Key1 = Value\r\nKey2 = Value\r\n\r\n[Section1]\r\nKey1 = Value\r\nKey2 = Value\r\n\r\n[Section2]\r\nKey1 = Value\r\nKey2 = Value\r\n",
      );
    } else {
      assert_eq!(
        str::from_utf8(&buf).unwrap(),
        "Key1 = Value\nKey2 = Value\n\n[Section1]\nKey1 = Value\nKey2 = Value\n\n[Section2]\nKey1 = Value\nKey2 = Value\n",
      );
    }
  }

  #[test]
  fn fix_issue64() {
    let input = format!("some-key = åäö{}", DEFAULT_LINE_SEPARATOR);

    let conf = Ltx::read_from_str(&input).unwrap();

    let mut output = Vec::new();
    conf.write_to(&mut output).unwrap();

    assert_eq!(input, String::from_utf8(output).unwrap());
  }
}
