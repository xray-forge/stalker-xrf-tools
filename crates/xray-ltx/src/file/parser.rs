use crate::file::configuration::constants::{
  LTX_SYMBOL_COMMENT, LTX_SYMBOL_INCLUDE, LTX_SYMBOL_INHERIT, LTX_SYMBOL_SECTION_CLOSE,
  LTX_SYMBOL_SECTION_OPEN,
};
use crate::file::configuration::line_separator::LineSeparator;
use crate::file::formatter::LtxFormatter;
use crate::file::section::section::Section;
use crate::file::section::section_entry::SectionEntry;
use crate::{Ltx, ROOT_SECTION};
use std::str::Chars;
use xray_error::{XRayError, XRayResult};

/// Ltx parser.
pub struct LtxParser<'a> {
  char: Option<char>,
  reader: Chars<'a>,
  line: usize,
  column: usize,
}

impl<'a> Default for LtxParser<'a> {
  fn default() -> Self {
    Self {
      char: None,
      line: 0,
      column: 0,
      reader: "".chars(),
    }
  }
}

impl<'a> LtxParser<'a> {
  /// Create new parser based on characters stream.
  pub fn new(reader: Chars<'a>) -> Self {
    let mut parser: Self = Self {
      char: None,
      line: 0,
      column: 0,
      reader,
    };

    parser.bump();

    parser
  }

  /// Parse the whole LTX input.
  pub fn parse(&mut self) -> XRayResult<Ltx> {
    let mut current_section: String = ROOT_SECTION.to_string();
    let mut includes_processed: bool = false;
    let mut ltx: Ltx = Ltx::new();

    self.skip_whitespaces();

    while let Some(current_char) = self.char {
      // Allow includes declaration header.
      // Allow writing comments before.
      if !includes_processed {
        includes_processed = !matches!(current_char, |LTX_SYMBOL_INCLUDE| LTX_SYMBOL_COMMENT)
      }

      match current_char {
        current if current == LTX_SYMBOL_COMMENT => {
          self.skip_comment()?;
        }

        current if current == LTX_SYMBOL_INCLUDE => {
          let line: String = self.parse_until_eol(true)?;
          let (included_path, _) = self.parse_include_from_line(&line)?;

          if ltx.includes(&included_path) {
            return self.error(format!(
              "Failed to parse include statement in ltx file, including '{}' more than once",
              &included_path
            ));
          } else {
            ltx.include(included_path)
          }
        }

        current if current == LTX_SYMBOL_SECTION_OPEN => {
          let line: String = self.parse_until_eol(true)?;
          let (section, inherited, _) = self.parse_section_from_line(&line)?;

          current_section = section;

          match ltx.entry(current_section.clone()) {
            SectionEntry::Vacant(vacant_entry) => {
              let mut properties: Section = Default::default();

              if let Some(inherited) = inherited {
                for base_name in inherited {
                  properties.inherit(base_name);
                }
              }

              vacant_entry.insert(properties);
            }
            SectionEntry::Occupied(_) => {
              return self.error(format!(
                "Duplicate sections are not allowed, looks like '{current_section}' is declared twice"
              ));
            }
          }
        }

        _ => {
          let line: String = self.parse_until_eol(true)?;
          let (key, value, _) = self.parse_key_value_from_line(&line)?;

          match ltx.entry(current_section.clone()) {
            SectionEntry::Vacant(vacant_entry) => {
              let mut properties: Section = Section::new();

              properties.insert(key, value.unwrap_or(String::new()));

              vacant_entry.insert(properties);
            }
            SectionEntry::Occupied(properties) => {
              properties
                .into_mut()
                .append(key, value.unwrap_or(String::new()));
            }
          }
        }
      }

      self.skip_whitespaces();
    }

    Ok(ltx)
  }

  /// Parse the whole LTX input and reformat as string.
  pub fn parse_into_formatted(&mut self) -> XRayResult<String> {
    let mut formatted: String = String::new();

    self.skip_whitespaces();

    while let Some(current_char) = self.char {
      let line: String = self.parse_until_eol(false)?;

      match current_char {
        current if current == LTX_SYMBOL_COMMENT => {
          LtxFormatter::write_comment(&mut formatted, &line[1..]);
        }

        current if current == LTX_SYMBOL_INCLUDE => {
          let (included_path, comment) = self.parse_include_from_line(&line)?;

          LtxFormatter::write_include(&mut formatted, &included_path, comment.as_deref());
        }

        current if current == LTX_SYMBOL_SECTION_OPEN => {
          let (section, inherited, comment) = self.parse_section_from_line(&line)?;

          LtxFormatter::write_section(&mut formatted, &section, inherited, comment.as_deref());
        }

        _ => {
          let (key, value, comment) = self.parse_key_value_from_line(&line)?;

          LtxFormatter::write_key_value(&mut formatted, &key, value.as_deref(), comment.as_deref());
        }
      }

      self.skip_whitespaces();
    }

    if !formatted.ends_with(LineSeparator::CRLF.as_str()) {
      formatted.push_str(LineSeparator::CRLF.as_str());
    }

    Ok(formatted)
  }

  /// Parse only include sections from file and return list of included LTX files.
  pub fn parse_includes(&mut self) -> XRayResult<Vec<String>> {
    let mut included: Vec<String> = Vec::new();

    self.skip_whitespaces();

    while let Some(current_char) = self.char {
      match current_char {
        current if current == LTX_SYMBOL_COMMENT => {
          self.skip_comment()?;
        }

        current if current == LTX_SYMBOL_INCLUDE => {
          let line: String = self.parse_until_eol(true)?;
          let (include_path, _) = self.parse_include_from_line(&line)?;

          if included.contains(&include_path) {
            return self.error(format!(
              "Failed to parse include statement in ltx file, including '{}' more than once",
              &include_path
            ));
          } else {
            included.push(include_path)
          }
        }

        _ => {
          return Ok(included);
        }
      }

      self.skip_whitespaces();
    }

    Ok(included)
  }
}

impl<'a> LtxParser<'a> {
  fn bump(&mut self) {
    self.char = self.reader.next();

    match self.char {
      Some('\n') => {
        self.line += 1;
        self.column = 0;
      }
      Some(..) => {
        self.column += 1;
      }
      None => {}
    }
  }

  /// Create parsing error.
  fn error<U, M: Into<String>>(&self, message: M) -> XRayResult<U> {
    Err(XRayError::new_ltx_parse_error(
      self.line + 1,
      self.column + 1,
      message,
    ))
  }

  /// Consume all the white space until the end of the line or a tab.
  fn skip_whitespaces(&mut self) {
    while let Some(char) = self.char {
      if !char.is_whitespace() && char != '\n' && char != '\t' && char != '\r' {
        break;
      }

      self.bump();
    }
  }

  /// Consume all the white space except line break.
  fn skip_whitespaces_except_line_break(&mut self) {
    while let Some(c) = self.char {
      if (c == '\n' || c == '\r' || !c.is_whitespace()) && c != '\t' {
        break;
      }

      self.bump();
    }
  }

  fn skip_comment(&mut self) -> XRayResult<String> {
    self.bump();

    // Allow empty value.
    self.skip_whitespaces_except_line_break();

    match self.char {
      None => Ok(String::new()),
      _ => self.parse_until_eol(false),
    }
  }

  fn parse_until(
    &mut self,
    endpoint: &[Option<char>],
    check_inline_comment: bool,
  ) -> XRayResult<String> {
    let mut result: String = String::new();

    while !endpoint.contains(&self.char) {
      match self.char {
        None => {
          return self.error(format!("Expecting \"{:?}\" but found EOF.", endpoint));
        }
        Some(space) if check_inline_comment && (space == ' ' || space == '\t') => {
          self.bump();

          match self.char {
            Some(';') => {
              // [space]; starts an inline comment.
              break;
            }
            Some(_) => {
              result.push(space);
              continue;
            }
            None => {
              result.push(space);
            }
          }
        }
        Some(c) => {
          result.push(c);
        }
      }
      self.bump();
    }

    let _ = check_inline_comment;
    Ok(result)
  }

  #[inline]
  fn parse_until_eol(&mut self, strip_inline_comment: bool) -> XRayResult<String> {
    let value: String = self.parse_until(&[Some('\n'), Some('\r'), None], strip_inline_comment)?;

    if strip_inline_comment && matches!(self.char, Some(LTX_SYMBOL_COMMENT)) {
      self.skip_comment()?;
    }

    Ok(value)
  }
}

impl<'a> LtxParser<'a> {
  /// Parse section name, inherited sections and comment from the line.
  fn parse_section_from_line(
    &self,
    line: &str,
  ) -> XRayResult<(String, Option<Vec<String>>, Option<String>)> {
    if line.is_empty() {
      return self.error("Failed to parse empty section statement");
    }

    let closing_bracket_position: Option<usize> = line.find(LTX_SYMBOL_SECTION_CLOSE);

    if closing_bracket_position.is_none() {
      return self.error("Failed to parse section statement without closing bracket ']'");
    }

    let section_ends_at: usize = closing_bracket_position.unwrap();
    let section: String = String::from(&line[1..section_ends_at]);
    let remainder: &str = line[section_ends_at + 1..].trim();

    if remainder.is_empty() {
      Ok((section, None, None))
    } else if let Some(remainder) = remainder.strip_prefix(LTX_SYMBOL_INHERIT) {
      let (inherited, comment) = match remainder.find(LTX_SYMBOL_COMMENT) {
        Some(position) => (&remainder[0..position], Some(&remainder[position + 1..])),
        None => (remainder, None),
      };

      let inherited: Vec<String> = inherited
        .split(',')
        .filter_map(|it| {
          let it: &str = it.trim();

          if it.is_empty() {
            None
          } else {
            Some(String::from(it))
          }
        })
        .collect::<Vec<String>>();

      Ok((
        section,
        if inherited.is_empty() {
          None
        } else {
          Some(inherited)
        },
        comment.map(|comment| String::from(comment.trim())),
      ))
    } else {
      // Fully trimmed value after splitting.
      let comment: String = String::from(remainder[1..].trim_start());

      Ok((
        section,
        None,
        if comment.is_empty() {
          None
        } else {
          Some(comment)
        },
      ))
    }
  }

  /// Parse section name, inherited sections and comment from the line.
  ///
  /// Supported include variants are:
  /// - #include "file.ltx"
  /// - #include("file.ltx")
  fn parse_include_from_line(&self, line: &str) -> XRayResult<(String, Option<String>)> {
    if line.is_empty() {
      return self.error("Failed to parse empty include statement");
    }

    let line: &str = line.trim();

    let (include, comment) = match line.split_once(';') {
      Some((key, value)) => (key.trim(), Some(value.trim())),
      None => (line, None),
    };

    let included_path: String = if include.starts_with("#include \"") && include.ends_with('\"') {
      String::from(&include[10..include.len() - 1])
    } else if include.starts_with("#include(\"") && include.ends_with("\")") {
      String::from(&include[10..include.len() - 2])
    } else if include.len() > 10 {
      if let Some(closing_index) = include[10..].find("\"") {
        // Closing index is -10 positions:
        String::from(&include[10..closing_index + 10])
      } else {
        return self.error(format!(
          "Expected correct '#include \"config.ltx\"' statement, got '{include}'"
        ));
      }
    } else {
      return self.error(format!(
        "Expected correct '#include \"config.ltx\"' statement, got '{include}'"
      ));
    };

    if included_path.is_empty() {
      return self.error(String::from(
        "Expected valid file name in include statement, got empty file name",
      ));
    }

    if !included_path.ends_with(".ltx") {
      return self.error(format!(
        "Included file should have .ltx extension, got '{included_path}'",
      ));
    }

    Ok((
      included_path,
      comment.filter(|it| !it.is_empty()).map(String::from),
    ))
  }

  /// Parse line key, value and comment from provided line.
  fn parse_key_value_from_line(
    &self,
    line: &str,
  ) -> XRayResult<(String, Option<String>, Option<String>)> {
    if line.is_empty() {
      return self.error("Failed to parse empty value statement");
    }

    let (data, comment) = match line.split_once(';') {
      None => (line.trim(), None),
      Some((data, comment)) => (data.trim(), Some(comment.trim())),
    };

    let (key, value) = match data.split_once('=') {
      None => (data.trim(), None),
      Some((key, value)) => (key.trim(), Some(value.trim())),
    };

    Ok((
      String::from(key),
      value.map(String::from),
      comment.filter(|it| !it.is_empty()).map(String::from),
    ))
  }
}

#[cfg(test)]
mod test {
  use crate::file::parser::LtxParser;

  #[test]
  fn test_read_section() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser.parse_section_from_line("[section]").unwrap(),
      (String::from("section"), None, None)
    );
  }

  #[test]
  fn test_read_section_with_inherited() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser
        .parse_section_from_line("[section] : a,   b, c")
        .unwrap(),
      (
        String::from("section"),
        Some(vec!(
          String::from("a"),
          String::from("b"),
          String::from("c"),
        )),
        None
      )
    );
  }

  #[test]
  fn test_read_section_with_empty_inherited() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser.parse_section_from_line("[section] :  ").unwrap(),
      (String::from("section"), None, None)
    );
  }

  #[test]
  fn test_read_section_with_empty_inherited_comment() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser
        .parse_section_from_line("[section] :  ;;;; test")
        .unwrap(),
      (
        String::from("section"),
        None,
        Some(String::from(";;; test"))
      )
    );
  }

  #[test]
  fn test_read_section_with_inherited_comment() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser
        .parse_section_from_line("[section] : a,  b    ;   commented phrase ")
        .unwrap(),
      (
        String::from("section"),
        Some(vec!(String::from("a"), String::from("b"))),
        Some(String::from("commented phrase"))
      )
    );
  }

  #[test]
  fn test_read_section_with_comment() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser
        .parse_section_from_line("[section];commented phrase ")
        .unwrap(),
      (
        String::from("section"),
        None,
        Some(String::from("commented phrase"))
      )
    );
  }

  #[test]
  fn test_read_key_value() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser
        .parse_key_value_from_line("  key   =   value")
        .unwrap(),
      (String::from("key"), Some(String::from("value")), None)
    );
  }

  #[test]
  fn test_read_key_value_comment() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser
        .parse_key_value_from_line("  key   =   1     ;   some phrase")
        .unwrap(),
      (
        String::from("key"),
        Some(String::from("1")),
        Some(String::from("some phrase"))
      )
    );
  }

  #[test]
  fn test_read_key_only() {
    let parser: LtxParser = Default::default();

    assert_eq!(
      parser.parse_key_value_from_line("  key   ").unwrap(),
      (String::from("key"), None, None)
    );
  }
}
