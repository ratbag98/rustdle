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
pub struct Grid {
    grid_letters: Vec<char>,
}

impl Grid {
    /// create a new Grid from a string slice
    pub fn new(letters: &str) -> Option<Grid> {
        println!("Grid new called with {}", letters);
        if square(letters.len()) && valid_letters(letters) {
            Some(Grid {
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
}
