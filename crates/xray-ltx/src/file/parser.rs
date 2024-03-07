use crate::file::configuration::constants::{
  LTX_SYMBOL_COMMENT, LTX_SYMBOL_INCLUDE, LTX_SYMBOL_INHERIT, LTX_SYMBOL_SECTION_CLOSE,
  LTX_SYMBOL_SECTION_OPEN,
};
use crate::file::configuration::line_separator::LineSeparator;
use crate::file::error::LtxParseError;
use crate::file::section_entry::SectionEntry;
use crate::{Ltx, Properties, ROOT_SECTION};
use std::str::Chars;

/// Ltx parser.
pub struct LtxParser<'a> {
  ch: Option<char>,
  rdr: Chars<'a>,
  line: usize,
  col: usize,
}

impl<'a> LtxParser<'a> {
  pub fn new(rdr: Chars<'a>) -> LtxParser<'a> {
    let mut parser: LtxParser = LtxParser {
      ch: None,
      line: 0,
      col: 0,
      rdr,
    };

    parser.bump();

    parser
  }

  fn bump(&mut self) {
    self.ch = self.rdr.next();

    match self.ch {
      Some('\n') => {
        self.line += 1;
        self.col = 0;
      }
      Some(..) => {
        self.col += 1;
      }
      None => {}
    }
  }

  fn error<U, M: Into<String>>(&self, message: M) -> Result<U, LtxParseError> {
    Err(LtxParseError {
      line: self.line + 1,
      col: self.col + 1,
      message: message.into(),
    })
  }

  /// Parse the whole LTX input.
  pub fn parse(&mut self) -> Result<Ltx, LtxParseError> {
    let mut includes_processed: bool = false;

    let mut ltx: Ltx = Ltx::new();
    let mut current_section: String = ROOT_SECTION.to_string();
    let mut current_key: String = String::new();

    self.parse_whitespace();

    while let Some(current_char) = self.ch {
      // Allow includes declaration header.
      // Allow writing comments before.
      if !includes_processed {
        includes_processed = current_char != '#' && current_char != ';';
      }

      match current_char {
        ';' => {
          self.parse_comment()?;
        }

        '#' => {
          if includes_processed {
            return self.error(String::from(
              "Unexpected '#include' statement, all include statements should be part of config heading",
            ));
          }

          match self.parse_str_until_eol(true) {
            Ok(include_line) => {
              let (include, _) = self.parse_include_from_line(&include_line)?;

              if ltx.includes(&include) {
                return self.error(format!(
                  "Failed to parse include statement in ltx file, including '{}' more than once",
                  &include
                ));
              } else {
                ltx.include(include)
              }
            }
            Err(error) => return Err(error),
          }
        }

        '[' => match self.parse_section(true) {
          Ok(section) => {
            current_section = String::from(section[..].trim());

            match ltx.entry(current_section.clone()) {
              SectionEntry::Vacant(vacant_entry) => {
                vacant_entry.insert(Default::default());
              }
              SectionEntry::Occupied(_) => {
                return self.error(String::from("Duplicate sections are not allowed"));
              }
            }
          }
          Err(error) => return Err(error),
        },

        '=' => {
          if current_key[..].is_empty() {
            return self.error("Missing key when parsing '=' in ltx file");
          }

          match self.parse_val(true) {
            Ok(value) => {
              let value: String = value[..].trim().to_owned();

              match ltx.entry(current_section.clone()) {
                SectionEntry::Vacant(vacant_entry) => {
                  let mut properties: Properties = Properties::new(); // cursec must be None (the General Section)

                  properties.insert(current_key, value);
                  vacant_entry.insert(properties);
                }
                SectionEntry::Occupied(occupied_entry) => {
                  // Insert into the last (current) section
                  occupied_entry.into_mut().append(current_key, value);
                }
              }
              current_key = String::new();
            }
            Err(error) => return Err(error),
          }
        }

        // Parsing of inherited sections.
        LTX_SYMBOL_INHERIT => match self.parse_val(true) {
          Ok(value) => {
            let value: String = value[..].trim().to_owned();

            match ltx.entry(current_section.clone()) {
              SectionEntry::Vacant(vacant_entry) => {
                let mut properties: Properties = Properties::new();

                self.inherit_from_string(&value, &mut properties);

                vacant_entry.insert(properties);
              }
              SectionEntry::Occupied(occupied_entry) => {
                self.inherit_from_string(&value, occupied_entry.into_mut());
              }
            }
            current_key = String::new();
          }
          Err(error) => return Err(error),
        },

        _ => match self.parse_key() {
          Ok(key) => {
            current_key = key[..].trim().to_owned();
          }
          Err(error) => return Err(error),
        },
      }

      self.parse_whitespace();
    }

    Ok(ltx)
  }

  /// Parse the whole LTX input and reformat as string.
  pub fn parse_into_formatted(&mut self) -> Result<String, LtxParseError> {
    self.parse_whitespace();

    let mut formatted: String = String::new();

    while let Some(current_char) = self.ch {
      match current_char {
        current if current == LTX_SYMBOL_COMMENT => {
          let comment_line: String = self.parse_str_until_eol(false)?;
          let comment: &str = comment_line[1..].trim_start();

          if !comment.is_empty() {
            formatted.push_str(&format!("; {comment}{}", LineSeparator::CRLF.as_str()));
          }
        }

        current if current == LTX_SYMBOL_INCLUDE => {
          let include_line: String = self.parse_str_until_eol(false)?;
          let (include_path, comment) = self.parse_include_from_line(&include_line)?;

          formatted.push_str(&format!("#include \"{include_path}\""));

          if let Some(comment) = comment {
            formatted.push_str(&format!(" ; {}", comment));
          }

          formatted.push_str(LineSeparator::CRLF.as_str());
        }

        current if current == LTX_SYMBOL_SECTION_OPEN => {
          let section_line: String = self.parse_str_until_eol(false)?;
          let (section, inherited, comment) = self.parse_section_from_line(&section_line)?;

          if !formatted.is_empty() {
            formatted.push_str(LineSeparator::CRLF.as_str())
          }

          formatted.push_str(&format!("[{section}]"));

          if let Some(inherited) = inherited {
            formatted.push_str(&format!(":{}", inherited.join(",")));
          }

          if let Some(comment) = comment {
            formatted.push_str(&format!(" ; {}", comment));
          }

          formatted.push_str(LineSeparator::CRLF.as_str());
        }

        _ => {
          let key_value_line: String = self.parse_str_until_eol(false)?;
          let (key, value, comment) = self.parse_key_value_from_line(&key_value_line)?;

          formatted.push_str(&key);

          if let Some(value) = value {
            if value.is_empty() {
              formatted.push_str(" =");
            } else {
              formatted.push_str(&format!(" = {value}"));
            }
          }

          if let Some(comment) = comment {
            formatted.push_str(&format!(" ; {comment}"));
          }

          formatted.push_str(LineSeparator::CRLF.as_str());
        }
      }

      self.parse_whitespace();
    }

    if !formatted.ends_with(LineSeparator::CRLF.as_str()) {
      formatted.push_str(LineSeparator::CRLF.as_str());
    }

    Ok(formatted)
  }

  /// Parse only include sections from file.
  pub fn parse_includes(&mut self) -> Result<Vec<String>, LtxParseError> {
    let mut includes_processed: bool = false;
    let mut includes: Vec<String> = Vec::new();

    self.parse_whitespace();

    while let Some(current_char) = self.ch {
      if !includes_processed {
        includes_processed =
          current_char != LTX_SYMBOL_INCLUDE && current_char != LTX_SYMBOL_COMMENT;
      }

      match current_char {
        ';' => {
          self.parse_comment()?;
        }

        '#' => {
          if includes_processed {
            return self.error(String::from(
              "Unexpected '#include' statement, all include statements should be part of config heading",
            ));
          }

          match self.parse_str_until_eol(true) {
            Ok(include_line) => {
              let (include, _) = self.parse_include_from_line(&include_line)?;

              if includes.contains(&include) {
                return self.error(format!(
                  "Failed to parse include statement in ltx file, including '{}' more than once",
                  &include
                ));
              } else {
                includes.push(include)
              }
            }
            Err(error) => return Err(error),
          }
        }
        _ => {
          return Ok(includes);
        }
      }

      self.parse_whitespace();
    }

    Ok(includes)
  }
}

impl<'a> LtxParser<'a> {
  fn inherit_from_string(&self, value: &str, properties: &mut Properties) {
    for base_name in value.split(',').map(|inherited| inherited.trim()) {
      if !base_name.is_empty() {
        properties.inherit(base_name);
      }
    }
  }

  /// Consume all the white space until the end of the line or a tab.
  fn parse_whitespace(&mut self) {
    while let Some(c) = self.ch {
      if !c.is_whitespace() && c != '\n' && c != '\t' && c != '\r' {
        break;
      }

      self.bump();
    }
  }

  /// Consume all the white space except line break.
  fn parse_whitespace_except_line_break(&mut self) {
    while let Some(c) = self.ch {
      if (c == '\n' || c == '\r' || !c.is_whitespace()) && c != '\t' {
        break;
      }

      self.bump();
    }
  }

  fn parse_comment(&mut self) -> Result<String, LtxParseError> {
    self.parse_val(false)
  }

  fn parse_str_until(
    &mut self,
    endpoint: &[Option<char>],
    check_inline_comment: bool,
  ) -> Result<String, LtxParseError> {
    let mut result: String = String::new();

    while !endpoint.contains(&self.ch) {
      match self.ch {
        None => {
          return self.error(format!("Expecting \"{:?}\" but found EOF.", endpoint));
        }
        Some(space) if check_inline_comment && (space == ' ' || space == '\t') => {
          self.bump();

          match self.ch {
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

  fn parse_section(&mut self, strip_comment: bool) -> Result<String, LtxParseError> {
    // Skip [
    self.bump();

    let section: String = self.parse_str_until(&[Some(LTX_SYMBOL_SECTION_CLOSE)], false)?;

    if let Some(LTX_SYMBOL_SECTION_CLOSE) = self.ch {
      self.bump();
    }

    // Deal with inline comment
    if strip_comment && matches!(self.ch, Some(LTX_SYMBOL_COMMENT)) {
      self.parse_comment()?;
    }

    Ok(section)
  }

  fn parse_key(&mut self) -> Result<String, LtxParseError> {
    self.parse_str_until(&[Some('='), Some('\n')], false)
  }

  fn parse_val(&mut self, strip_inline_comment: bool) -> Result<String, LtxParseError> {
    self.bump();

    // Allow empty value.
    self.parse_whitespace_except_line_break();

    match self.ch {
      None => Ok(String::new()),
      _ => self.parse_str_until_eol(strip_inline_comment),
    }
  }

  #[inline]
  fn parse_str_until_eol(&mut self, strip_inline_comment: bool) -> Result<String, LtxParseError> {
    let value: String =
      self.parse_str_until(&[Some('\n'), Some('\r'), None], strip_inline_comment)?;

    if strip_inline_comment && matches!(self.ch, Some(LTX_SYMBOL_COMMENT)) {
      self.parse_comment()?;
    }

    Ok(value)
  }
}

impl<'a> LtxParser<'a> {
  /// Parse section name, inherited sections and comment from the line.
  fn parse_section_from_line(
    &self,
    line: &str,
  ) -> Result<(String, Option<Vec<String>>, Option<String>), LtxParseError> {
    if line.is_empty() {
      return self.error("Failed to parse empty section statement");
    }

    let closing_bracket_position: Option<usize> = line.find(']');

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
  fn parse_include_from_line(&self, line: &str) -> Result<(String, Option<String>), LtxParseError> {
    if line.is_empty() {
      return self.error("Failed to parse empty include statement");
    }

    let line: &str = line.trim();

    let (include, comment) = match line.split_once(';') {
      Some((key, value)) => (key.trim(), Some(value.trim())),
      None => (line, None),
    };

    if !include.starts_with("#include \"") || !include.ends_with('\"') {
      return self.error(format!(
        "Expected correct '#include \"config.ltx\"' statement, got '{include}'"
      ));
    }

    let included_path: String = String::from(&include[10..include.len() - 1]);

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
  ) -> Result<(String, Option<String>, Option<String>), LtxParseError> {
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

impl<'a> Default for LtxParser<'a> {
  fn default() -> LtxParser<'a> {
    LtxParser {
      ch: None,
      line: 0,
      col: 0,
      rdr: "".chars(),
    }
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
