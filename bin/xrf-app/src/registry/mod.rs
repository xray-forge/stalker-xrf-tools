//! Command registration, derived from one token list.
//!
//! `domains.rs` pairs every wire name with its Rust command path. `runtime.rs` expands that list into a module
//! per domain, re-exported here so a domain reads as `crate::registry::<domain>`; `build.rs` expands the same
//! list into the inline plugin and ACL declarations the build script needs.

#[macro_use]
mod domains;
mod runtime;

pub(crate) use runtime::*;
