const LUA_HELP_SCRIPT_PATH: &str = r"scripts\lua_help.script";

/// Returns whether a script asset contains executable Lua used by the game runtime.
///
/// `lua_help.script` is an engine API declaration generated for editor tooling. Its
/// `global`, `namespace`, and `class` declarations are not LuaJIT syntax, so it
/// must not be passed to the runtime Lua parser.
pub fn is_runtime_script(path: &str) -> bool {
  !path.eq_ignore_ascii_case(LUA_HELP_SCRIPT_PATH)
}

#[cfg(test)]
mod tests {
  use super::is_runtime_script;

  #[test]
  fn excludes_lua_help_from_runtime_script_verification() {
    assert!(!is_runtime_script(r"scripts\lua_help.script"));
    assert!(!is_runtime_script(r"SCRIPTS\LUA_HELP.SCRIPT"));
    assert!(is_runtime_script(r"scripts\bind_stalker.script"));
  }
}
