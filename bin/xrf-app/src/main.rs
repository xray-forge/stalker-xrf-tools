// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod application;
#[cfg(all(test, feature = "typescript-bindings"))]
mod bindings;
mod plugins;
mod registry;

use crate::app::logging::setup_logger;

fn main() {
  setup_logger();
  application::run();
}
