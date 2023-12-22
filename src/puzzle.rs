//! A Squaredle puzzle
use num_integer::{Integer, Roots};

/// is a grid of letters "square" (l x l)?
fn square<T: Integer + Roots + Copy>(size: T) -> bool {
    let root = size.sqrt();

    // sqrt truncates to the nearest integer so this checks for squareness
    root * root == size
}

#[derive(Debug)]
/// a grid of letters, stored as Vec.
pub struct Grid {
    grid_letters: Vec<char>,
}

impl Grid {
    /// create a new Grid from a string slice
    pub fn new(letters: &str) -> Option<Grid> {
        if square(letters.len()) {
            Some(Grid {
                grid_letters: letters.chars().collect(),
            })
        } else {
            None
        }
    }
    /// retrieve all the letters from the Grid
    pub fn letters(self) -> String {
        self.grid_letters.into_iter().collect()
    }
}
