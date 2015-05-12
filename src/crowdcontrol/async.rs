//! Primitives for asynchronous computation.
//!
//! At the time of this writing, Rust lacks agreed upon mechanisms
//! of asynchronous control flow. This module provides some basics heavily
//! influenced by popular variants of Futures in other languages (e.g.,
//! Guava Futures, twitter-util Futures, Scala futures).
//!
//! The module is likely to be replaced eventually as an alternative shows
//! up. In the mean time, this module is likely far from optimal and represents
//! more of a rust learning opportunity for the author than a serious attempt at
//! generally applicablie rust futures.

/// A temporal duration in nanoseconds.
///
/// XXX(scode): Likely replaced by std::time::Duration or similar in the future,
///             once there is a stable type to use.
pub type Duration = i64;

/// XXX(scode): Hard choices.
///
/// I struggled to decide whether to conflate asynchronicity and failure handling. At
/// a high level, I considered these options:
///
///   (a) Have Future always be failable (ala Scala futures).
///   (b) Have no concept of failures of futures at all.
///   (c) Have non-failable Future and associated traits implemented for
///       Future<Result<T, E>>.
///   (d) Separate Future and Failable future, embracing the conflation in
///       the latter case.
///
/// The reason I rejected (a) was that it was not clear how to represent the notion
/// that a future can *not* fail, other than using some sentinel error type which seems
/// unclean.
///
/// The reason I rejected (b) was that certain key combinators require knowledge of
/// failures - such as a short-circuiting collect().
///
/// The reason I rejected (c) is that it introduces ambiguity - given a Future<Result<Int,IOError>>,
/// is that supposed to be treated like a failable future with value type Int, or a non-failable
/// future whose value happens to be a Result<Int,IOError>? Invoking something like "map" should not
/// "accidentally" result in treating a non-failable future like a failable future.
///
/// (d) Seems to avoid these problems, yet feels a bit awkward.

// TODO(scode): Callbacks missing.
// TODO(scode): Combinators missing.

/// Represents the result of an asynchronous computation that cannot fail (other
/// than by panicing the process).
pub trait Future<T: Copy + Send> {
    /// Return Some(value) if the future has been satisfied, else None.
    fn get(self: &Self) -> Option<T>;

    /// Wait until the future has been satisfied, and return its value.
    fn await_forever(self: &Self) -> T;

    /// Wait until the future has been satisfied, or the specified timeout. If the future becomes
    /// satisfied prior to the timeout, Some(value) is returned. Else, None is returned to indicate
    /// a timeout occurred.
    ///
    /// XXX(scode): Use an Error to represent the timeout, to facilitate better debugging (e.g., include
    ///             duration of the timeout that expired in a description)?
    fn await_until(self: &Self, timeout: Duration) -> Option<T>;
}

/// Represents the result of an asynchronous computation that can fail.
///
/// XXX(scode): Not convinced we should extend Future, vs having a separate interface. One could
///             also make a case to conflate asynchronicity and failures in a single Future trait.
pub trait FailableFuture<T: Copy + Send, E: Copy + Send> : Future<Result<T, E>> {
    /// Return Some(result) if the future has been satisfied, else None.
    fn fget(self: &Self) -> Option<Result<T, E>>;

    fn fawait_forever(self: &Self) -> Result<T, E>;
    fn fawait_until(self: &Self, timeout: Duration) -> Option<Result<T, E>>;
}
