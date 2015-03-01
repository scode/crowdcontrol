// XXX(scode): Consider supporting cloning/snapshotting metrics.
// XXX(scode): Consider whether we should expose the current values in the public traits.
// XXX(scode): Missing histograms.
// XXX(scode): Missing meters (questionable).
// XXX(scode): Transition to lock-less where possible.

use std::num::Int;
use std::sync::Arc;
use std::sync::Mutex;

/// A 64 bit signed counter.
pub trait Counter<T: Int> {
    /// Increment the counter by the given delta.
    ///
    // The delta may be negative (for types T where this is possible).
    fn inc(&mut self, delta: T);

    /// Decrement the counter by the given delta.
    ///
    /// The delta may be negative (for types T where this is possible).
    ///
    /// dec(n) is equivalent of dec(-n)
    fn dec(&mut self, delta: T);
}

/// A gauge has a single value at any given moment in time, and can only be
/// updated by providing an entirely new value to replace the existing one
/// (if any).
pub trait Gauge<T> {
    fn set(&mut self, value: T);
}

pub struct SimpleCounter<T: Int> {
    value: T,
}

impl<T: Int> Counter<T> for SimpleCounter<T> {
    fn inc(&mut self, delta: T) {
        self.value = self.value + delta;
    }

    fn dec(&mut self, delta: T) {
        self.value = self.value - delta;
    }
}

pub struct SharedCounter<T: Int + Send> {
    value: Arc<Mutex<T>>,
}

impl<T: Int + Send> Counter<T> for SharedCounter<T> {
    fn inc(&mut self, delta: T) {
        let mut value = self.value.lock().unwrap();

        *value = *value + delta;
    }

    fn dec(&mut self, delta: T) {
        let mut value = self.value.lock().unwrap();

        *value = *value - delta;
    }
}

pub struct SimpleGauge<T> {
    value: Option<T>,
}

impl<T> Gauge<T> for SimpleGauge<T> {
    fn set(&mut self, new_value: T) {
        self.value = Some(new_value);
    }
}

pub struct SharedGauge<T: Send> {
    value: Arc<Mutex<Option<T>>>,
}

impl<T: Send> Gauge<T> for SharedGauge<T> {
    fn set(&mut self, new_value: T) {
        *self.value.lock().unwrap() = Some(new_value)
    }
}

#[cfg(test)]
mod test {
    // TODO(scode): Add tests.
}

