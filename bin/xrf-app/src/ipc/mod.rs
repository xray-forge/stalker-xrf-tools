//! The command surface the webview calls into, and everything derived from it.
//!
//! `registry` declares every command once and expands that list into runtime dispatch, ACL entries, and Specta
//! collections. `bindings` turns the same surface into the frontend's TypeScript mirrors, and exists only in a
//! test build.

#[cfg(all(test, feature = "typescript-bindings"))]
pub(crate) mod bindings;
pub(crate) mod registry;
