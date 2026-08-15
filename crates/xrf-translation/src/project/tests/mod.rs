mod edit;
mod gamedata_read;
mod layout;
mod source_read;

/// A one-entry string table, which is all most of these tests need on disk.
fn table(id: &str, text: &str) -> String {
  format!("<string_table>\n\t<string id=\"{id}\">\n\t\t<text>{text}</text>\n\t</string>\n</string_table>")
}
