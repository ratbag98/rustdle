// #![deny(elided_lifetimes_in_paths)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(warnings)]

//! Solve a Squaredle puzzle using rust

use basic_trie::DatalessTrie;
use clap::{Args, Parser, ValueEnum};
use std::fmt;
use std::fs;

/// different sources for the puzzle's letters
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PuzzleType {
    StandardPuzzle,
    ExpressPuzzle,
}

impl fmt::Display for PuzzleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            PuzzleType::StandardPuzzle => write!(f, "{}", "standard-puzzle"),
            PuzzleType::ExpressPuzzle => write!(f, "{}", "express-puzzle"),
        }
    }
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct LetterSelection {
    /// User-specified grid of letters
    #[arg(long)]
    letters: Option<String>,

    /// Create a random square grid of x by x letters
    #[arg(short = 'x', long)]
    square: Option<u8>,

    /// download puzzle, either standard or express
    #[arg(short, long, default_value_t = PuzzleType::StandardPuzzle)]
    download: PuzzleType,
}

#[derive(Parser, Debug)]
#[command(author = "Rob Rainthorpe", version, about = "Solve Squaredles with the power of Rust!", long_about = None)]
struct RustdleArgs {
    #[command(flatten)]
    letter_selection: LetterSelection,

    /// display results as a single column
    #[arg(short = 'c', long, default_value_t = false)]
    single_column: bool,

    /// display letters grid
    #[arg(short, long, default_value_t = false)]
    grid: bool,

    /// group solutions by length
    #[arg(short, long, default_value_t = false)]
    length: bool,

    /// display headers for length-grouped solution lists
    #[arg(short = 'H', long, default_value_t = false)]
    headers: bool,

    /// show all solutions for a word in GUI
    #[arg(short, long, default_value_t = false)]
    multiple: bool,

    /// randomise letter order, maybe useful for setting puzzles
    #[arg(short, long, default_value_t = false)]
    random: bool,

    /// sort solutions alphabetically
    #[arg(short, long, default_value_t = true)]
    sort: bool,
}

fn main() {
    let args = RustdleArgs::parse();
    println!("Arguments: {:?}", args);

    let valid_words = fs::read_to_string("./word_list.txt").expect("problem reading word list");

    let mut trie = DatalessTrie::new();

    let words = valid_words.lines();

    let mut count = 0;
    for word in words {
        trie.insert(word);
        count += 1;
    }

    let search = "ROBE";

    let found_words = trie.find_words(search).unwrap();
    let found_missing_word = trie.contains("NONENSENOTAWORD");

    assert!(found_words.len() > 0);
    assert!(found_missing_word == false);

    println!("Valid words starting {search}: {:?}", found_words);

    print!("Wordlist has {} bytes.", valid_words.len());
    print!("Wordlist has {} words.", count);
}
