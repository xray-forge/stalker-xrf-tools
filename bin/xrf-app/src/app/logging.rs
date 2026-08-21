use std::env;

use env_logger::Builder;
use log::LevelFilter;
use tauri::Runtime;
use tauri::ipc::Invoke;

/// Configure environment logger, fallback to info level.
pub fn setup_logger() {
  let mut logger: Builder = env_logger::builder();

  if let Ok(rust_log) = env::var("RUST_LOG") {
    logger.parse_filters(&rust_log);
  } else {
    // Debug builds trace every command dispatch; release keeps the info lines the commands already
    // write, which is what makes a user's log useful in a bug report. At the previous warn/error
    // levels none of them were ever emitted, so the logging in the commands did nothing at all.
    match cfg!(debug_assertions) {
      true => logger.filter_level(LevelFilter::Debug),
      false => logger.filter_level(LevelFilter::Info),
    };
  }

  logger.default_format();
  logger.init();
}

/// Wrap a generated invoke handler so a command the plugin does not define is reported.
pub fn warn_on_unhandled_command<R, F>(
  plugin: &'static str,
  handler: F,
) -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static
where
  R: Runtime,
  F: Fn(Invoke<R>) -> bool + Send + Sync + 'static,
{
  move |invoke: Invoke<R>| {
    let command: String = invoke.message.command().to_string();

    let handled: bool = handler(invoke);

    if !handled {
      log::warn!("Plugin '{plugin}' was asked for command '{command}', which it does not define");
    }

    handled
  }
}
