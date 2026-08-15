/// Parsing behavior shared by X-Ray XML readers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlParseOptions {
  /// Whether a document declaring a DTD is accepted. Off by default, because a DTD is an input the
  /// engine never writes and a parser feature worth not exposing to untrusted game data.
  pub allow_dtd: bool,
}
