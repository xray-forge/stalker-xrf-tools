mod application;
mod commands;
mod core;
mod registry;

use crate::core::logging::setup_logger;

fn main() {
  setup_logger();
  application::run();
}
