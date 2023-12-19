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
            PuzzleType::StandardPuzzle => write!(f, "{}", "standard"),
            PuzzleType::ExpressPuzzle => write!(f, "{}", "express"),
        }
    }
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct LetterSelection {
    /// User-specified grid of letters
    #[arg(short, long, conflicts_with = "download")]
    letters: Option<String>,

    /// download puzzle, either standard or express
    #[arg(short, long, conflicts_with = "letters",default_value_t = PuzzleType::StandardPuzzle)]
    download: PuzzleType,
}

#[derive(Parser, Debug)]
#[command(author = "Rob Rainthorpe", version, about = "Solve Squaredles with the power of Rust!", long_about = None)]
struct SquaredleArgs {
    #[command(flatten)]
    letter_selection: LetterSelection,

    /// Create a random square grid of x by x letters
    #[arg(short = 'x', long)]
    square: Option<u8>,
}

fn main() {
    let args = SquaredleArgs::parse();
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
