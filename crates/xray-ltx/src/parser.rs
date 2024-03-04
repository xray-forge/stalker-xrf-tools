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
  // Create a parser.
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

  fn error<U, M: Into<String>>(&self, msg: M) -> Result<U, LtxParseError> {
    Err(LtxParseError {
      line: self.line + 1,
      col: self.col + 1,
      msg: msg.into(),
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
    let mut result = Ltx::new();
    let mut curkey: String = "".into();
    let mut cursec: Option<String> = None;

    self.parse_whitespace();
    while let Some(cur_ch) = self.ch {
      match cur_ch {
        ';' | '#' => {
          self.parse_comment();
        }
        '[' => match self.parse_section() {
          Ok(sec) => {
            let msec = sec[..].trim();
            cursec = Some((*msec).to_string());
            match result.entry(cursec.clone()) {
              SectionEntry::Vacant(v) => {
                v.insert(Default::default());
              }
              SectionEntry::Occupied(mut o) => {
                o.append(Default::default());
              }
            }
          }
          Err(e) => return Err(e),
        },
        '=' | ':' => {
          if (curkey[..]).is_empty() {
            return self.error("missing key");
          }
          match self.parse_val() {
            Ok(val) => {
              let mval = val[..].trim().to_owned();
              match result.entry(cursec.clone()) {
                SectionEntry::Vacant(v) => {
                  // cursec must be None (the General Section)
                  let mut prop = Properties::new();
                  prop.insert(curkey, mval);
                  v.insert(prop);
                }
                SectionEntry::Occupied(mut o) => {
                  // Insert into the last (current) section
                  o.last_mut().append(curkey, mval);
                }
              }
              curkey = "".into();
            }
            Err(e) => return Err(e),
          }
        }
        _ => match self.parse_key() {
          Ok(key) => {
            let mkey: String = key[..].trim().to_owned();
            curkey = mkey;
          }
          Err(e) => return Err(e),
        },
      }

      self.parse_whitespace();
    }

    Ok(result)
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
