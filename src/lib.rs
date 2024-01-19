//! Support functions for rustdle
use std::fmt;

/// Parse command line arguments
pub mod cli;

/// store a puzzle grid
pub mod puzzle;

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
        match self {
            PuzzleType::Standard => write!(f, "standard"),
            PuzzleType::Express => write!(f, "express"),
        }
    }
}

/// solve a puzzle
pub mod solver;
