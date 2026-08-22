/// The language key standing for text that is not translated per language.
///
/// An XML source with no language suffix is copied to every language by the build, so calling it
/// English - which is what a filename fallback would do - would present a change to all languages as
/// a change to one.
pub const LANGUAGE_NEUTRAL: &str = "all";

/// The infix marking the merged view of several language-suffixed XML sources.
pub const MULTILANGUAGE: &str = "multilang";

/// Directory the engine excludes from languages by name rather than by content.
///
/// `CStringTable::FillLanguageToken` skips it explicitly, so a tool that reads languages by listing
/// directories has to skip it too or it invents a language the game does not have.
pub(crate) const MAP_DESC_DIRECTORY: &str = "map_desc";

/// A language directory holding only this is not a language, by the engine's own rule.
pub(crate) const OPENXRAY_XML: &str = "openxray.xml";
