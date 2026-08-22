use std::cmp::max;
use std::time::{Duration, Instant};

/// The clock and the counter one unpacking run reports itself with.
///
/// Held apart from the runs because the sequential and the concurrent one differ only in how they drive the writes:
/// timing the two phases and deciding how often to log are one story, and two copies of it had already drifted on
/// which entries they counted.
pub(crate) struct ArchiveUnpackProgress {
  started_at: Instant,
  /// How long creating the destination tree took, which precedes any payload being written.
  prepared_at: Duration,
  completed: usize,
  total: usize,
  /// Entries between progress lines, so a large set logs about twenty times and a small one still says something.
  step: usize,
}

impl ArchiveUnpackProgress {
  /// Starts the clock, before the destination tree is prepared.
  pub(crate) fn begin(total: usize) -> Self {
    Self {
      started_at: Instant::now(),
      prepared_at: Duration::ZERO,
      completed: 0,
      total,
      step: max(total / 100 * 5, 5),
    }
  }

  /// Closes the preparation phase, once the destination tree exists.
  pub(crate) fn record_prepared(&mut self) {
    self.prepared_at = self.started_at.elapsed();
  }

  /// Counts one entry as dealt with, whether it was written or had nothing to write.
  pub(crate) fn record_unpacked(&mut self) {
    self.completed += 1;

    if self.completed.is_multiple_of(self.step) {
      log::info!("Unpacked {}/{} files", self.completed, self.total);
    }
  }

  /// How long preparing the destination tree took.
  pub(crate) fn get_prepared_at(&self) -> Duration {
    self.prepared_at
  }

  /// How long the run has taken so far, preparation included.
  pub(crate) fn elapsed(&self) -> Duration {
    self.started_at.elapsed()
  }
}
