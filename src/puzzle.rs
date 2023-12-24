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
    pub fn letters(self) -> String {
        self.grid_letters.into_iter().collect()
    }

    /// compose set of unique letters in grid
    pub fn unique_letters(self) -> String {
        // want the letters sorted so BTree rather than Hash
        let mut unique_letters: BTreeSet<char> = BTreeSet::new();
        for l in self.grid_letters {
            unique_letters.insert(l);
        }
        let result = unique_letters.into_iter().collect();
        println!("Unique returned {}", result);
        result
    }
}
