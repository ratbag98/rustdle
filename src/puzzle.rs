//! A Squaredle puzzle
use num_integer::{Integer, Roots};
use regex::Regex;
use std::collections::BTreeSet;

/// is a grid of letters "square" (l x l)?
fn square<T: Integer + Roots + Copy>(size: T) -> bool {
    let root = size.sqrt();

    // sqrt truncates to the nearest integer so this checks for squareness
    root * root == size
}

/// are provided letters letters or underscores only?
fn valid_letters(letters: &str) -> bool {
    // next to impossible for hte new to fail, so I'll unwrap
    let acceptable = Regex::new(r"^[_A-Z]+$").unwrap();

    acceptable.is_match(letters)
}

#[derive(Debug)]
/// a grid of letters, stored as Vec.
pub struct Puzzle {
    grid_letters: Vec<char>,
}

impl Puzzle {
    /// create a new Grid from a string slice
    pub fn new(letters: &str) -> Option<Self> {
        println!("Grid new called with {}", letters);
        if square(letters.len()) && valid_letters(letters) {
            Some(Self {
                grid_letters: letters.chars().collect(),
            })
        } else {
            None
        }
    }

    /// retrieve all the letters from the Grid
    pub fn letters(&self) -> String {
        self.grid_letters.iter().collect()
    }

    /// compose set of unique letters in grid
    pub fn unique_letters(&self) -> String {
        self.grid_letters
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// calculate length of the sides of the grid
    pub fn side_length(self) -> u8 {
        // by this stage, we know that the grid is a square number
        // so don't obsess about ints, floors, etc., just trust the maths
        self.grid_letters.len().sqrt().try_into().unwrap()
    }
}
