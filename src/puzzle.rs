//! A Squaredle puzzle

#[derive(Debug)]
/// a grid of letters, stored as Vec.
pub struct Grid {
    grid_letters: Vec<char>,
}

impl Grid {
    /// create a new Grid from a string slice
    pub fn new(letters: &str) -> Grid {
        Grid {
            grid_letters: letters.chars().collect(),
        }
    }
    /// retrieve all the letters from the Grid
    pub fn letters(self) -> String {
        self.grid_letters.into_iter().collect()
    }
}
