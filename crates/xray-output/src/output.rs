use std::fmt::Display;

/// Renders live, user-facing workflow messages.
pub trait Output: Send + Sync {
  fn heading(&self, message: &dyn Display);

  fn success(&self, message: &dyn Display);

  fn warning(&self, message: &dyn Display);

  fn failure(&self, message: &dyn Display);

  fn info(&self, message: &dyn Display);

  fn error(&self, message: &dyn Display);

  fn verbose(&self, message: &dyn Display);
}

/// Discards every workflow message.
#[derive(Default)]
pub struct NoopOutput;

impl Output for NoopOutput {
  fn heading(&self, _: &dyn Display) {}

  fn success(&self, _: &dyn Display) {}

  fn warning(&self, _: &dyn Display) {}

  fn failure(&self, _: &dyn Display) {}

  fn info(&self, _: &dyn Display) {}

  fn error(&self, _: &dyn Display) {}

  fn verbose(&self, _: &dyn Display) {}
}
