use crate::escape_policy::escape_str;
use crate::{EscapePolicy, Ltx, WriteOption};
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

impl Ltx {
  /// Write to a file
  pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> io::Result<()> {
    self.write_to_file_policy(filename, EscapePolicy::Basics)
  }

  /// Write to a file
  pub fn write_to_file_policy<P: AsRef<Path>>(
    &self,
    filename: P,
    policy: EscapePolicy,
  ) -> io::Result<()> {
    let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(filename.as_ref())?;
    self.write_to_policy(&mut file, policy)
  }

  /// Write to a file with options
  pub fn write_to_file_opt<P: AsRef<Path>>(&self, filename: P, opt: WriteOption) -> io::Result<()> {
    let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(filename.as_ref())?;
    self.write_to_opt(&mut file, opt)
  }

  /// Write to a writer
  pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
    self.write_to_opt(writer, Default::default())
  }

  /// Write to a writer
  pub fn write_to_policy<W: Write>(&self, writer: &mut W, policy: EscapePolicy) -> io::Result<()> {
    self.write_to_opt(
      writer,
      WriteOption {
        escape_policy: policy,
        ..Default::default()
      },
    )
  }

  /// Write to a writer with options
  pub fn write_to_opt<W: Write>(&self, writer: &mut W, option: WriteOption) -> io::Result<()> {
    let mut firstline = true;

    // Write include statements.
    if !self.includes.is_empty() {
      firstline = false;

      for include in &self.includes {
        write!(writer, "#include \"{}\"{}", include, option.line_separator)?;
      }
    }

    for (section, props) in &self.sections {
      // If root section with data or generic section.
      if section.is_some() || !props.data.is_empty() {
        if firstline {
          firstline = false;
        } else {
          // Write an empty line between sections
          writer.write_all(option.line_separator.as_str().as_bytes())?;
        }
      }

      if let Some(ref section) = *section {
        let inherited: String = if props.inherited.is_empty() {
          String::new()
        } else {
          format!(
            ":{}",
            props
              .inherited
              .iter()
              .filter_map(|it| {
                if it.is_some() {
                  Some(it.clone().unwrap().into_inner())
                } else {
                  None
                }
              })
              .collect::<Vec<String>>()
              .join(", ")
          )
        };

        write!(
          writer,
          "[{}]{}{}",
          escape_str(&section[..], option.escape_policy),
          escape_str(&inherited, option.escape_policy),
          option.line_separator
        )?;
      }

      for (k, v) in props.iter() {
        let key_string = escape_str(k, option.escape_policy);
        let value_string = escape_str(v, option.escape_policy);

        write!(
          writer,
          "{}{}{}{}",
          key_string, option.kv_separator, value_string, option.line_separator
        )?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::line_separator::{LineSeparator, DEFAULT_LINE_SEPARATOR};
  use crate::{EscapePolicy, Ltx, WriteOption};

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
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let mut buf = vec![];
    ltx.write_to(&mut buf).unwrap();
    let new_data = Ltx::load_from_str(&String::from_utf8(buf).unwrap()).unwrap();

    let sec0 = new_data.general_section();
    let keys0: Vec<&str> = sec0.iter().map(|(k, _)| k).collect();
    assert_eq!(keys0, vec!["x2", "x1", "x3"]);

    let sec1 = new_data.section(Some("s")).unwrap();
    let keys1: Vec<&str> = sec1.iter().map(|(k, _)| k).collect();
    assert_eq!(keys1, vec!["x2", "xb", "a3"]);
  }

  #[test]
  fn write_new() {
    use std::str;

    let ltx: Ltx = Ltx::new();

    let opt = WriteOption {
      line_separator: LineSeparator::CR,
      ..Default::default()
    };
    let mut buf = Vec::new();
    ltx.write_to_opt(&mut buf, opt).unwrap();

    assert_eq!("", str::from_utf8(&buf).unwrap());
  }

  #[test]
  fn write_line_separator() {
    use std::str;

    let mut ini = Ltx::new();
    ini
      .with_section(Some("Section1"))
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section(Some("Section2"))
      .set("Key1", "Value")
      .set("Key2", "Value");

    {
      let mut buf = Vec::new();
      ini
        .write_to_opt(
          &mut buf,
          WriteOption {
            line_separator: LineSeparator::CR,
            ..Default::default()
          },
        )
        .unwrap();

      assert_eq!(
        "[Section1]\nKey1=Value\nKey2=Value\n\n[Section2]\nKey1=Value\nKey2=Value\n",
        str::from_utf8(&buf).unwrap()
      );
    }

    {
      let mut buf = Vec::new();
      ini
        .write_to_opt(
          &mut buf,
          WriteOption {
            line_separator: LineSeparator::CRLF,
            ..Default::default()
          },
        )
        .unwrap();

      assert_eq!(
        "[Section1]\r\nKey1=Value\r\nKey2=Value\r\n\r\n[Section2]\r\nKey1=Value\r\nKey2=Value\r\n",
        str::from_utf8(&buf).unwrap()
      );
    }

    {
      let mut buf = Vec::new();
      ini
        .write_to_opt(
          &mut buf,
          WriteOption {
            line_separator: LineSeparator::SystemDefault,
            ..Default::default()
          },
        )
        .unwrap();

      if cfg!(windows) {
        assert_eq!(
          "[Section1]\r\nKey1=Value\r\nKey2=Value\r\n\r\n[Section2]\r\nKey1=Value\r\nKey2=Value\r\n",
          str::from_utf8(&buf).unwrap()
        );
      } else {
        assert_eq!(
          "[Section1]\nKey1=Value\nKey2=Value\n\n[Section2]\nKey1=Value\nKey2=Value\n",
          str::from_utf8(&buf).unwrap()
        );
      }
    }
  }

  #[test]
  fn write_kv_separator() {
    use std::str;

    let mut ini = Ltx::new();

    ini
      .with_section(None::<String>)
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section(Some("Section1"))
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section(Some("Section2"))
      .set("Key1", "Value")
      .set("Key2", "Value");

    let mut buf = Vec::new();
    ini
      .write_to_opt(
        &mut buf,
        WriteOption {
          kv_separator: " = ",
          ..Default::default()
        },
      )
      .unwrap();

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
    let input = format!("some-key=åäö{}", DEFAULT_LINE_SEPARATOR);

    let conf = Ltx::load_from_str(&input).unwrap();

    let mut output = Vec::new();
    conf
      .write_to_policy(&mut output, EscapePolicy::Basics)
      .unwrap();

    assert_eq!(input, String::from_utf8(output).unwrap());
  }
}
