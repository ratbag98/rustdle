//! A Squaredle puzzle
use num_integer::{Integer, Roots};
use regex::Regex;
use std::fmt;

/// is a grid of letters "square" (l x l)?
fn square<T: Integer + Roots + Copy>(size: T) -> bool {
    let root = size.sqrt();

    // sqrt truncates to the nearest integer so this checks for squareness
    root * root == size
}

/// at least 4 characters, all letters or underscores?
fn valid_letters(letters: &str) -> bool {
    // next to impossible for hte new to fail, so I'll unwrap
    let valid = Regex::new(r"^[_A-Z]{4,}$").unwrap();

    valid.is_match(letters)
}

/// big enough, good letters, square puzzle?
fn ok_to_create(letters: &str) -> bool {
    valid_letters(letters) && square(letters.len())
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

    bodge_length: isize,
}

impl Puzzle {
    /// create a new Grid from a string slice
    pub fn new(letters: &str) -> Option<Self> {
        let side_length = letters.len().sqrt();
        let bodge_length: isize = isize::try_from(side_length).unwrap();
        if ok_to_create(letters) {
            Some(Self {
                letters: letters.chars().collect(),
                bodge_length,
                side_length,
            })
        } else {
            None
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
        // .iter()
        // .collect::<BTreeSet<_>>()
        // .into_iter()
        // .collect()
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
    pub fn on_grid(&self, row: &isize, col: &isize) -> bool {
        (0..self.bodge_length).contains(row) && (0..self.bodge_length).contains(col)
    }

    /// return the linear indexes of the letters surrounding the
    /// provided coordinates
    pub fn neighbours_of(&self, row: usize, col: usize) -> Vec<usize> {
        let i_row = isize::try_from(row).unwrap();
        let i_col = isize::try_from(col).unwrap();

        let del: [isize; 3] = [-1, 0, 1];

        let mut neighbours = vec![];
        for y in del {
            for x in del {
                if x == 0 && y == 0 {
                    continue;
                }
                let n_row = i_row + y;
                let n_col = i_col + x;
                if self.on_grid(&n_row, &n_col) {
                    println!("({:?},{:?})", n_row, n_col);
                    neighbours.push(self.index_of(n_row as usize, n_col as usize));
                }
            }
        }
        neighbours
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
