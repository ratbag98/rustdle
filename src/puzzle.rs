//! A Squaredle puzzle
use num_integer::Roots;
use regex::Regex;
use std::fmt;

/// at least 4 characters, all letters or underscores?
fn valid_letters(letters: &str) -> bool {
    // next to impossible for hte new to fail, so I'll unwrap
    let valid = Regex::new(r"^[_A-Z]{4,}$").unwrap();

    valid.is_match(letters)
}

#[derive(Debug)]
/// a grid of letters, stored as Vec.
/// we can manipulate it as a Grid using index() and row()
pub struct Puzzle {
    /// the letters in the puzzle reading from top to bottom
    /// without separators
    letters: Vec<char>,

    /// the length of the side of the square when the Puzzle
    /// is shown as a grid
    side_length: usize,
}

impl Puzzle {
    /// create a new Grid from a string slice
    pub fn new(letters: &str) -> Result<Self, &'static str> {
        let side_length = letters.len().sqrt();

        // not square
        if side_length * side_length != letters.len() {
            return Err("can't make a square grid from that many letters.");
        }

        if valid_letters(letters) {
            Ok(Self {
                letters: letters.chars().collect(),
                side_length,
            })
        } else {
            Err("letters must be A-Z or underscore (_).")
        }
    }

    /// retrieve all the letters from the Grid
    pub fn letters(&self) -> String {
        self.letters.iter().collect()
    }

    /// compose set of unique letters in grid
    pub fn unique_letters(&self) -> String {
        let mut chars = self.letters.to_vec();
        chars.sort();
        chars.dedup();
        chars.iter().collect()
    }

    /// return the letter at a particular row and column
    pub fn letter_at(&self, row: usize, col: usize) -> &char {
        let i = self.side_length * row;
        &self.letters[i + col]
    }

    /// the linear index of the given cell coordinates
    pub fn index_of(&self, row: usize, col: usize) -> usize {
        (&self.side_length * row) + col
    }

    /// return a row from the Puzzle
    pub fn row(&self, row: usize) -> &[char] {
        let i = self.side_length * row;
        &self.letters[i..(i + self.side_length)]
    }

    /// is a cell inside the grid
    pub fn on_grid(&self, row: usize, col: usize) -> bool {
        row < self.side_length && col < self.side_length
    }

    /// return the linear indexes of the letters surrounding the
    /// provided coordinates
    pub fn neighbours_of(&self, row: usize, col: usize) -> Vec<usize> {
        let offsets: [i8; 3] = [-1, 0, 1];

        let mut neighbours = vec![];
        for y in offsets {
            for x in offsets {
                // not a neighbour of myself!
                if x == 0 && y == 0 {
                    continue;
                }
                if let Some(n_row) = self.constrained_add(row, y) {
                    if let Some(n_col) = self.constrained_add(col, x) {
                        if self.on_grid(n_row, n_col) {
                            println!("Adding {}, {} as neighbour", n_row, n_col);
                            neighbours.push(self.index_of(n_row, n_col));
                        }
                    }
                }
            }
        }
        neighbours
    }

    /// add an offset (-1, 0 or 1) to an unsigned value and return a value if result is greater
    /// than zero
    fn constrained_add(&self, u: usize, offset: i8) -> Option<usize> {
        match offset {
            0 => Some(u),
            1 => Some(u.checked_add(1)?),
            -1 => Some(u.checked_sub(1)?),
            _ => None,
        }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the letters as a grid
        for row in self.letters.chunks(self.side_length) {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
