#[macro_use]
mod registry;

macro_rules! define_runtime_domains {
  (
    $(
      $domain:ident => $plugin_name:literal {
        $($command_name:ident => $command_head:ident $(:: $command_tail:ident)*,)*
      }
    )*
  ) => {
    $(
      pub(crate) mod $domain {
        use tauri::Runtime;
        use tauri::ipc::Invoke;

        pub(crate) const NAME: &str = $plugin_name;

        pub(crate) fn handler<R: Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
          tauri::generate_handler![$($command_head $(:: $command_tail)*),*]
        }

        #[cfg(feature = "typescript-bindings")]
        pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
          tauri_specta::Builder::new()
            .plugin_name(NAME)
            .error_handling(tauri_specta::ErrorHandlingMode::Throw)
            .commands(tauri_specta::collect_commands![$($command_head $(:: $command_tail)*),*])
            .disable_serde_phases()
            .dangerously_cast_bigints_to_number()
        }
      }
    )*
  };
}

for_each_tauri_command_domain!(define_runtime_domains);
