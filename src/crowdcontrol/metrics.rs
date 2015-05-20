//! Metrics mechanisms intended to provide runtime visibility.

// XXX(scode): Consider re-using third party metrics library.
// XXX(scode): Consider supporting cloning/snapshotting metrics.
// XXX(scode): Missing histograms.
// XXX(scode): Missing meters (questionable).
// XXX(scode): Transition to lock-less where possible.
// XXX(scode): Support exposing metrics beyond the local process.

use num::integer::Integer;
use std::sync::Arc;
use std::sync::Mutex;

/// A 64 bit signed counter.
pub trait Counter<T: Integer> {
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

pub struct SimpleCounter<T: Integer> {
    value: T,
}

impl<T: Integer + Clone> Counter<T> for SimpleCounter<T> {
    fn inc(&mut self, delta: T) {
        self.value = self.value.clone() + delta;
    }

    fn dec(&mut self, delta: T) {
        self.value = self.value.clone() - delta;
    }

    fn get(&self) -> T {
        self.value.clone()
    }
}

pub struct SharedCounter<T: Integer + Send> {
    value: Arc<Mutex<T>>,
}

impl<T: Integer + Clone + Send> Counter<T> for SharedCounter<T> {
    fn inc(&mut self, delta: T) {
        let mut value = self.value.lock().unwrap();

        *value = value.clone() + delta;
    }

    fn dec(&mut self, delta: T) {
        let mut value = self.value.lock().unwrap();

        *value = value.clone() - delta;
    }

    fn get(&self) -> T {
        let value = self.value.lock().unwrap();

        value.clone()
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

    #[test]
    fn test_gauge() {
        use metrics::Gauge;
        use metrics::SimpleGauge;
        use metrics::SharedGauge;

        let mut gauges = Vec::<Box<Gauge<i64>>>::new();
        gauges.push(Box::new(SimpleGauge { value: Some(0i64), }));
        gauges.push(Box::new(SharedGauge { value: Arc::new(Mutex::new(Some(0i64)))}));

        for mut g in gauges {
            assert_eq!(g.get(), Some(0));
            g.set(Some(5));
            assert_eq!(g.get(), Some(5))
        }
    }
}

