use cfg_if::cfg_if;
use std::fmt::{Display, Error, Formatter};
use instant::{Instant, Duration};

cfg_if! {
    if #[cfg(all(
        target_arch = "wasm32",
        target_os = "unknown",
        target_vendor = "unknown"
    ))] {
        use core::ops::Add;

        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        struct DummyInstant(Duration);

        impl DummyInstant {
            pub fn now() -> DummyInstant {
                DummyInstant::zero()
            }

            const fn zero() -> DummyInstant {
                DummyInstant(Duration::from_secs(0))
            }
        }

        impl Add<Duration> for DummyInstant {
            type Output = DummyInstant;

            fn add(self, _rhs: Duration) -> DummyInstant {
                DummyInstant::zero()
            }
        }

        // Use dummy implementation for `Instant` on `wasm32`. The reason for this is
        // that `Instant::now()` will always panic because time is currently not implemented
        // on wasm32-unknown-unknown.
        // See https://github.com/rust-lang/rust/blob/master/src/libstd/sys/wasm/time.rs
        type InstantType = DummyInstant;
    } else {
        // Otherwise use `std::time::Instant`
        type InstantType = Instant;
    }
}

/// A timer.
#[derive(Copy, Clone, Debug, Default)]
pub struct Timer {
    enabled: bool,
    time: f64,
    start: Option<f64>,
}

impl Timer {
    /// Creates a new timer initialized to zero and not started.
    pub fn new() -> Self {
        Timer {
            enabled: false,
            time: 0.0,
            start: None,
        }
    }

    /// Enables this timer.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables this timer.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Resets the timer to 0.
    pub fn reset(&mut self) {
        self.time = 0.0
    }

    /// Start the timer.
    pub fn start(&mut self) {
        if self.enabled {
            self.time = 0.0;
            self.start = Some(instant::now());
        }
    }

    /// Pause the timer.
    pub fn pause(&mut self) {
        if self.enabled {
            if let Some(start) = self.start {
                self.time += instant::now() - start;
            }
            self.start = None;
        }
    }

    /// Resume the timer.
    pub fn resume(&mut self) {
        if self.enabled {
            self.start = Some(instant::now());
        }
    }

    /// The measured time between the last `.start()` and `.pause()` calls.
    pub fn time(&self) -> f64 {
        self.time
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}s", self.time)
    }
}