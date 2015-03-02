// XXX(scode): Consider supporting cloning/snapshotting metrics.
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

    /// Return the current value of the counter.
    fn get(&self) -> T;
}

/// A gauge has a single value at any given moment in time, and can only be
/// updated by providing an entirely new value.
pub trait Gauge<T: Clone> {
    fn set(&mut self, value: Option<T>);
    fn get(&self) -> Option<T>;
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

    fn get(&self) -> T {
        self.value
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

    fn get(&self) -> T {
        let value = self.value.lock().unwrap();

        *value
    }
}

pub struct SimpleGauge<T: Clone> {
    value: Option<T>,
}

impl<T: Clone> Gauge<T> for SimpleGauge<T> {
    fn set(&mut self, new_value: Option<T>) {
        self.value = new_value;
    }

    fn get(&self) -> Option<T> {
        self.value.clone()
    }
}

pub struct SharedGauge<T: Send> {
    value: Arc<Mutex<Option<T>>>,
}

impl<T: Clone + Send> Gauge<T> for SharedGauge<T> {
    fn set(&mut self, new_value: Option<T>) {
        *self.value.lock().unwrap() = new_value;
    }

    fn get(&self) -> Option<T> {
        (*self.value.lock().unwrap()).clone()
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_counters() {
        use metrics::Counter;
        use metrics::SharedCounter;
        use metrics::SimpleCounter;

        let mut counters = Vec::<Box<Counter<i64>>>::new();
        counters.push(Box::new(SimpleCounter { value: 0i64, }));
        counters.push(Box::new(SharedCounter { value: Arc::new(Mutex::new(0i64)) }));

        for mut c in counters {
            assert_eq!(c.get(), 0);
            c.inc(1);
            assert_eq!(c.get(), 1);
            c.dec(1);
            assert_eq!(c.get(), 0);

            c.dec(-1);
            assert_eq!(c.get(), 1);
            c.inc(-1);
            assert_eq!(c.get(), 0);
        }
    }

    // TODO(scode): Test gauge.
}

