//! Local persistent journal of records.
//!
//! This module contains abstractions intended to manage a local
//! persistent (in the durable sense) journal, shielding the user
//! from various file system related implementation details while
//! exposing mechanisms and guarantees generally suitable to ensure
//! correctness while allowing for decent performance.

use std::cmp;

/// A position in a journal, potentially identifying a particular
/// record.
///
/// Positions should always be obtained from a Journal and never
/// instantiated directly.
#[derive(PartialOrd,PartialEq,Eq,Clone,Copy,Debug)]
pub struct Position {
    // Currently this represents a position in a file. In the future,
    // we expect to want to represent additional information such as
    // a journal segment identifier. As such, we are being maximally
    // careful about exactly what information we expose in the public
    // interface.
    position: u64,
}

impl cmp::Ord for Position {
    fn cmp(self: &Position, other: &Position) -> cmp::Ordering {
        return self.position.cmp(&other.position)
    }
}

/// A journal represents a strictly ordered sequence of records.
///
/// A journal guarantees the following:
///
///  - Records are persistently stored once confirmed written.
///  - Once written, a record has a well-defined order exposed through
///    its position (which is allocated on writing).
///  - For any two records with sequence ids A and B, where A < B,
///    if B is persisted, so is A. Further, if B is readable, so is
///    A.
pub trait Journal {
    // TODO(scode): Finish.
}

#[cfg(test)]
mod test {
    use journal;

    fn pos(pos: u64) -> journal::Position {
        journal::Position { position: pos }
    }

    #[test]
    fn test_ord() {
        assert_eq!(pos(0), pos(0));
        assert!(pos(0) < pos(1));
        assert!(pos(1) > pos(0));
    }
}
