/// Rewrite the constructs X-Ray accepts and XML does not, without moving a single byte.
///
/// Every substitution replaces one byte with one byte, so offsets into the result are offsets into
/// the input. That is the whole point: the repaired copy is only ever parsed, and every range it
/// yields still addresses the original text, so an edit splices what was actually on disk.
pub(crate) fn repair_for_parsing(input: &str) -> String {
  let mut repaired: Vec<u8> = input.as_bytes().to_vec();
  let bytes: &[u8] = input.as_bytes();
  let mut index: usize = 0;

  while index < bytes.len() {
    if bytes[index..].starts_with(b"<!--") {
      let body: usize = index + 4;
      let end: usize = find(bytes, body, b"-->").unwrap_or(bytes.len());

      // A comment body may not contain `--`, and shipped banners are made of little else. Blanking
      // every dash in the body is same-length and cannot break the terminator, which sits outside it.
      for byte in &mut repaired[body..end] {
        if *byte == b'-' {
          *byte = b'~';
        }
      }

      index = end.saturating_add(3).min(bytes.len());

      continue;
    }

    // A bare ampersand is not a reference, and translation text is full of them.
    if bytes[index] == b'&' && !is_reference_at(bytes, index) {
      repaired[index] = b'~';
    }

    index += 1;
  }

  String::from_utf8(repaired).unwrap_or_else(|_| input.to_owned())
}

/// Whether the ampersand at `start` begins something shaped like an entity reference.
fn is_reference_at(bytes: &[u8], start: usize) -> bool {
  let mut index: usize = start + 1;

  if bytes.get(index) == Some(&b'#') {
    index += 1;
  }

  let name_start: usize = index;

  while let Some(byte) = bytes.get(index) {
    match byte {
      b';' => return index > name_start,
      byte if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-') => index += 1,
      _ => return false,
    }
  }

  false
}

fn find(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
  bytes
    .get(from..)?
    .windows(needle.len())
    .position(|window| window == needle)
    .map(|offset| from + offset)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn every_repair_keeps_the_input_length() {
    // The one invariant everything else depends on: same length means offsets still line up.
    for source in [
      "<!-- ---- names ---- -->",
      "<text>Smith & Wesson</text>",
      "<text>a &amp; b</text>",
      "<root/>",
    ] {
      assert_eq!(repair_for_parsing(source).len(), source.len(), "for {source}");
    }
  }

  #[test]
  fn blanks_dashes_inside_a_comment_body() {
    assert_eq!(repair_for_parsing("<!-- -- -->"), "<!-- ~~ -->");
  }

  #[test]
  fn leaves_the_comment_terminator_intact() {
    // Eating the closing dashes would leave the comment unterminated and unparseable.
    assert!(repair_for_parsing("<!------->").ends_with("-->"));
  }

  #[test]
  fn neutralizes_a_bare_ampersand() {
    assert_eq!(repair_for_parsing("a & b"), "a ~ b");
  }

  #[test]
  fn leaves_a_real_entity_alone() {
    assert_eq!(repair_for_parsing("a &amp; b"), "a &amp; b");
    assert_eq!(repair_for_parsing("a &#38; b"), "a &#38; b");
  }

  #[test]
  fn treats_an_unterminated_entity_as_a_bare_ampersand() {
    assert_eq!(repair_for_parsing("a &amp b"), "a ~amp b");
  }

  #[test]
  fn leaves_a_well_formed_document_untouched() {
    let source: &str = "<root><child id=\"a\">text</child></root>";

    assert_eq!(repair_for_parsing(source), source);
  }
}
