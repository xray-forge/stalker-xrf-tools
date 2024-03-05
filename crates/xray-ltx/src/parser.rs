use crate::error::LtxParseError;
use crate::ltx::Ltx;
use crate::parse_option::ParseOption;
use crate::properties::Properties;
use crate::section_entry::SectionEntry;
use std::str::Chars;

// Ltx parser.
pub struct LtxParser<'a> {
  ch: Option<char>,
  rdr: Chars<'a>,
  line: usize,
  col: usize,
  opt: ParseOption,
}

impl<'a> LtxParser<'a> {
  pub fn new(rdr: Chars<'a>, opt: ParseOption) -> LtxParser<'a> {
    let mut p = LtxParser {
      ch: None,
      line: 0,
      col: 0,
      rdr,
      opt,
    };
    p.bump();
    p
  }

  fn eof(&self) -> bool {
    self.ch.is_none()
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

  /// Parse the whole LTX input.
  pub fn parse(&mut self) -> Result<Ltx, LtxParseError> {
    let mut includes_processed: bool = false;

    let mut ltx: Ltx = Ltx::new();
    let mut current_key: String = "".into();
    let mut current_section: Option<String> = None;

    self.parse_whitespace();

    while let Some(current_char) = self.ch {
      if !includes_processed {
        includes_processed = current_char != '#';
      }

      match current_char {
        ';' => {
          self.parse_comment();
        }

        '#' => {
          if includes_processed {
            return self.error(String::from(
              "Unexpected '#include' statement, all include statements should be part of config heading",
            ));
          }

          match self.parse_include() {
            Ok(value) => {
              if ltx.includes(&value) {
                return self.error(format!(
                  "Failed to parse include statement in ltx file, including '{}' more than once",
                  &value
                ));
              } else {
                ltx.include(value)
              }
            }
            Err(error) => return Err(error),
          }
        }

        '[' => match self.parse_section() {
          Ok(sec) => {
            let member_section = sec[..].trim();

            current_section = Some((*member_section).to_string());

            match ltx.entry(current_section.clone()) {
              SectionEntry::Vacant(vacant_entry) => {
                vacant_entry.insert(Default::default());
              }
              SectionEntry::Occupied(mut occupied_entry) => {
                occupied_entry.append(Default::default());
              }
            }
          }
          Err(error) => return Err(error),
        },

        '=' => {
          if (current_key[..]).is_empty() {
            return self.error("missing key");
          }

          match self.parse_val() {
            Ok(value) => {
              let value: String = value[..].trim().to_owned();

              match ltx.entry(current_section.clone()) {
                SectionEntry::Vacant(vacant_entry) => {
                  let mut properties = Properties::new(); // cursec must be None (the General Section)
                  properties.insert(current_key, value);
                  vacant_entry.insert(properties);
                }
                SectionEntry::Occupied(mut occupied_entry) => {
                  // Insert into the last (current) section
                  occupied_entry.last_mut().append(current_key, value);
                }
              }
              current_key = String::new();
            }
            Err(error) => return Err(error),
          }
        }

        // Parsing of inherited sections.
        ':' => match self.parse_val() {
          Ok(value) => {
            let value: String = value[..].trim().to_owned();

            match ltx.entry(current_section.clone()) {
              SectionEntry::Vacant(vacant_entry) => {
                let mut properties: Properties = Properties::new();

                for base_name in value.split(',').map(|it| it.trim()) {
                  properties.inherit(Some(base_name));
                }

                vacant_entry.insert(properties);
              }
              SectionEntry::Occupied(mut occupied_entry) => {
                let properties: &mut Properties = occupied_entry.last_mut();

                for base_name in value.split(',').map(|it| it.trim()) {
                  properties.inherit(Some(base_name));
                }
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

  fn parse_comment(&mut self) {
    while let Some(c) = self.ch {
      self.bump();
      if c == '\n' {
        break;
      }
    }
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
          return self.error(format!("expecting \"{:?}\" but found EOF.", endpoint));
        }
        Some(space) if check_inline_comment && (space == ' ' || space == '\t') => {
          self.bump();

          match self.ch {
            Some('#') | Some(';') => {
              // [space]#, [space]; starts an inline comment.
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
        Some('\\') if self.opt.enabled_escape => {
          self.bump();
          if self.eof() {
            return self.error(format!("expecting \"{:?}\" but found EOF.", endpoint));
          }
          match self.ch.unwrap() {
            '0' => result.push('\0'),
            'a' => result.push('\x07'),
            'b' => result.push('\x08'),
            't' => result.push('\t'),
            'r' => result.push('\r'),
            'n' => result.push('\n'),
            '\n' => (),
            'x' => {
              // Unicode 4 character
              let mut code: String = String::with_capacity(4);
              for _ in 0..4 {
                self.bump();
                if self.eof() {
                  return self.error(format!("expecting \"{:?}\" but found EOF.", endpoint));
                } else if let Some('\\') = self.ch {
                  self.bump();
                  if self.ch != Some('\n') {
                    return self.error(format!(
                      "expecting \"\\\\n\" but \
                                             found \"{:?}\".",
                      self.ch
                    ));
                  }
                }
                code.push(self.ch.unwrap());
              }
              let r = u32::from_str_radix(&code[..], 16);
              match r {
                Ok(c) => match char::from_u32(c) {
                  Some(c) => result.push(c),
                  None => {
                    return self.error("unknown character in \\xHH form");
                  }
                },
                Err(_) => return self.error("unknown character in \\xHH form"),
              }
            }
            c => result.push(c),
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

  fn parse_section(&mut self) -> Result<String, LtxParseError> {
    // Skip [
    self.bump();
    let sec = self.parse_str_until(&[Some(']')], false)?;
    if let Some(']') = self.ch {
      self.bump();
    }

    // Deal with inline comment
    if matches!(self.ch, Some('#') | Some(';')) {
      self.parse_comment();
    }

    Ok(sec)
  }

  fn parse_key(&mut self) -> Result<String, LtxParseError> {
    self.parse_str_until(&[Some('='), Some(':')], false)
  }

  fn parse_val(&mut self) -> Result<String, LtxParseError> {
    self.bump();
    // Issue #35: Allow empty value
    self.parse_whitespace_except_line_break();

    match self.ch {
      None => Ok(String::new()),
      Some('"') if self.opt.enabled_quote => {
        self.bump();
        self.parse_str_until(&[Some('"')], false).and_then(|s| {
          self.bump(); // Eats the last "
                       // Parse until EOL
          self.parse_str_until_eol(true).map(|x| s + &x)
        })
      }
      Some('\'') if self.opt.enabled_quote => {
        self.bump();
        self.parse_str_until(&[Some('\'')], false).and_then(|s| {
          self.bump(); // Eats the last '
                       // Parse until EOL
          self.parse_str_until_eol(true).map(|x| s + &x)
        })
      }
      _ => self.parse_str_until_eol(true),
    }
  }

  fn parse_include(&mut self) -> Result<String, LtxParseError> {
    let value: String = self.parse_val()?[..].trim().to_owned();

    if !value.starts_with("include \"") || !value.ends_with('\"') {
      return self.error(format!(
        "Expected correct '#include \"config.ltx\"' statement, got '#{value}'"
      ));
    }

    let value: String = String::from(&value[9..value.len() - 1]);

    if value.is_empty() {
      return self.error(format!(
        "Expected valid file name in include statement, got empty file name",
      ));
    }

    if !value.ends_with(".ltx") {
      return self.error(format!(
        "Included file should have .ltx extension, got '{value}'",
      ));
    }

    Ok(value)
  }

  #[inline]
  fn parse_str_until_eol(&mut self, check_inline_comment: bool) -> Result<String, LtxParseError> {
    let value: String =
      self.parse_str_until(&[Some('\n'), Some('\r'), None], check_inline_comment)?;

    if check_inline_comment && matches!(self.ch, Some('#') | Some(';')) {
      self.parse_comment();
    }

    Ok(value)
  }
}
