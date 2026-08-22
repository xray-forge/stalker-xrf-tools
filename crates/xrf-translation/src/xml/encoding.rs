use std::fs;
use std::path::Path;

use xrf_error::{XrfError, XrfResult};
use xrf_utils::{XRayEncoding, decode_bytes_to_string_without_bom_handling, new_utf8_encoder};
use xrf_xml::declared_xml_encoding;

use crate::language::TranslationLanguage;

/// Decide which encoding a string table file is written in.
///
/// Shared by reading and writing on purpose: resolving it differently in each direction would decode
/// a file one way and re-encode it another, corrupting every byte an edit never touched.
///
/// The declaration wins where there is one. Otherwise the language comes from the filename suffix,
/// then from the parent directory, which is where raw gamedata carries it.
///
/// # Errors
///
/// Returns an encoding error when the declaration names a code page there is no encoder for.
pub(crate) fn resolve_encoding(path: &Path, data: &[u8]) -> XrfResult<XRayEncoding> {
  let language: Option<TranslationLanguage> =
    TranslationLanguage::from_file_name(path).or_else(|| TranslationLanguage::from_parent_directory(path));
  let declared: Option<XRayEncoding> = declared_xml_encoding(data)?;

  if let (Some(declared), Some(language)) = (declared, language)
    && declared != language.new_language_encoder()
  {
    log::warn!(
      "Translation XML '{}' declares {}, but '{}' expects {}",
      path.display(),
      declared.name(),
      language,
      language.get_language_encoding(),
    );
  }

  Ok(declared.unwrap_or_else(|| language.unwrap_or(TranslationLanguage::English).new_language_encoder()))
}

/// Read a string table into text, with the byte order mark and encoding needed to write it back.
///
/// Reading and writing share this so a file is always decoded and re-encoded the same way.
///
/// # Errors
///
/// Returns an IO error when the file cannot be read, and an encoding error when its bytes do not
/// decode as the encoding it claims.
pub(crate) fn read_decoded(path: &Path) -> XrfResult<(Vec<u8>, XRayEncoding, String)> {
  let data: Vec<u8> = fs::read(path)?;
  let (mark, body) = split_byte_order_mark(path, &data)?;

  // A byte order mark decides the encoding and outranks the declaration, which shipped files
  // contradict: gamedata-coc's st_items_weapons.xml is UTF-8 marked and declares windows-1251. It is
  // also kept out of the decoded text and put back verbatim, because the usual decode strips it and
  // re-encoding would then drop it from a file that had one.
  let encoding: XRayEncoding = if mark.is_empty() {
    resolve_encoding(path, body)?
  } else {
    new_utf8_encoder()
  };

  Ok((
    mark.to_vec(),
    encoding,
    decode_bytes_to_string_without_bom_handling(body, encoding)?,
  ))
}

/// Split a leading byte order mark off the content, so it survives a rewrite untouched.
///
/// # Errors
///
/// Returns an encoding error for UTF-16 content, which none of the string table encoders can hold
/// and which decoding as a code page would silently mangle.
fn split_byte_order_mark<'a>(path: &Path, data: &'a [u8]) -> XrfResult<(&'a [u8], &'a [u8])> {
  if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
    return Ok(data.split_at(3));
  }

  if data.starts_with(&[0xFF, 0xFE]) || data.starts_with(&[0xFE, 0xFF]) {
    return Err(XrfError::new_encoding_error(format!(
      "Translation '{}' is UTF-16, which string tables cannot be written as",
      path.display()
    )));
  }

  Ok((&[], data))
}
