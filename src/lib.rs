//! Support functions for rustdle
use std::fmt;

/// Parse command line arguments
pub mod cli;

/// different sources for the puzzle's letters
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PuzzleType {
    /// Download today's main puzzle
    Standard,

    /// Download today's Express (small) puzzle
    Express,
}

impl fmt::Display for PuzzleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            PuzzleType::Standard => write!(f, "standard"),
            PuzzleType::Express => write!(f, "express"),
        }
    }
}
