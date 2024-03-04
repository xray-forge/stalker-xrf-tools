/// Policies for escaping logic.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EscapePolicy {
  /// Escape absolutely nothing (dangerous)
  Nothing,
  /// Only escape the most necessary things.
  /// This means backslashes, control characters (codepoints U+0000 to U+001F), and delete (U+007F).
  /// Quotes (single or double) are not escaped.
  Basics,
  /// Escape basics and non-ASCII characters in the [Basic Multilingual Plane](https://www.compart.com/en/unicode/plane)
  /// (i.e. between U+007F - U+FFFF)
  /// Codepoints above U+FFFF, e.g. '🐱' U+1F431 "CAT FACE" will *not* be escaped!
  BasicsUnicode,
  /// Escape basics and all non-ASCII characters, including codepoints above U+FFFF.
  /// This will escape emoji - if you want them to remain raw, use BasicsUnicode instead.
  BasicsUnicodeExtended,
  /// Escape reserved symbols.
  /// This includes everything in EscapePolicy::Basics, plus the comment characters ';'
  /// and '#' and the key/value-separating characters '=' and ':'.
  Reserved,
  /// Escape reserved symbols and non-ASCII characters in the BMP.
  /// Codepoints above U+FFFF, e.g. '🐱' U+1F431 "CAT FACE" will *not* be escaped!
  ReservedUnicode,
  /// Escape reserved symbols and all non-ASCII characters, including codepoints above U+FFFF.
  ReservedUnicodeExtended,
  /// Escape everything that some LTX implementations assume.
  Everything,
}

impl EscapePolicy {
  fn escape_basics(self) -> bool {
    self != EscapePolicy::Nothing
  }

  fn escape_reserved(self) -> bool {
    matches!(
      self,
      EscapePolicy::Reserved
        | EscapePolicy::ReservedUnicode
        | EscapePolicy::ReservedUnicodeExtended
        | EscapePolicy::Everything
    )
  }

  fn escape_unicode(self) -> bool {
    matches!(
      self,
      EscapePolicy::BasicsUnicode
        | EscapePolicy::BasicsUnicodeExtended
        | EscapePolicy::ReservedUnicode
        | EscapePolicy::ReservedUnicodeExtended
        | EscapePolicy::Everything
    )
  }

  fn escape_unicode_extended(self) -> bool {
    matches!(
      self,
      EscapePolicy::BasicsUnicodeExtended
        | EscapePolicy::ReservedUnicodeExtended
        | EscapePolicy::Everything
    )
  }

  /// Given a character this returns true if it should be escaped as
  /// per this policy or false if not.
  pub fn should_escape(self, c: char) -> bool {
    match c {
      // A single backslash, must be escaped
      // ASCII control characters, U+0000 NUL..= U+001F UNIT SEPARATOR, or U+007F DELETE. The same as char::is_ascii_control()
      '\\' | '\x00'..='\x1f' | '\x7f' => self.escape_basics(),
      ';' | '#' | '=' | ':' => self.escape_reserved(),
      '\u{0080}'..='\u{FFFF}' => self.escape_unicode(),
      '\u{10000}'..='\u{10FFFF}' => self.escape_unicode_extended(),
      _ => false,
    }
  }
}

// Escape non-LTX characters.
//
// Common escape sequences: https://en.wikipedia.org/wiki/INI_file#Escape_characters
//
// * `\\` \ (a single backslash, escaping the escape character)
// * `\0` Null character
// * `\a` Bell/Alert/Audible
// * `\b` Backspace, Bell character for some applications
// * `\t` Tab character
// * `\r` Carriage return
// * `\n` Line feed
// * `\;` Semicolon
// * `\#` Number sign
// * `\=` Equals sign
// * `\:` Colon
// * `\x????` Unicode character with hexadecimal code point corresponding to ????
pub fn escape_str(s: &str, policy: EscapePolicy) -> String {
  let mut escaped: String = String::with_capacity(s.len());
  for c in s.chars() {
    // if we know this is not something to escape as per policy, we just write it and continue.
    if !policy.should_escape(c) {
      escaped.push(c);
      continue;
    }

    match c {
      '\\' => escaped.push_str("\\\\"),
      '\0' => escaped.push_str("\\0"),
      '\x01'..='\x06' | '\x0e'..='\x1f' | '\x7f'..='\u{00ff}' => {
        escaped.push_str(&format!("\\x{:04x}", c as isize)[..])
      }
      '\x07' => escaped.push_str("\\a"),
      '\x08' => escaped.push_str("\\b"),
      '\x0c' => escaped.push_str("\\f"),
      '\x0b' => escaped.push_str("\\v"),
      '\n' => escaped.push_str("\\n"),
      '\t' => escaped.push_str("\\t"),
      '\r' => escaped.push_str("\\r"),
      '\u{0080}'..='\u{FFFF}' => escaped.push_str(&format!("\\x{:04x}", c as isize)[..]),
      // Longer escapes.
      '\u{10000}'..='\u{FFFFF}' => escaped.push_str(&format!("\\x{:05x}", c as isize)[..]),
      '\u{100000}'..='\u{10FFFF}' => escaped.push_str(&format!("\\x{:06x}", c as isize)[..]),
      _ => {
        escaped.push('\\');
        escaped.push(c);
      }
    }
  }
  escaped
}
