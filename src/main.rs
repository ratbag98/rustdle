//! Solve a Squaredle puzzle using rust

use clap::Parser;
use rustdle::cli::RustdleArgs;
use rustdle::puzzle::Puzzle;

use basic_trie::DatalessTrie;
use std::fs;

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

    let letters = match &args.letter_selection.letters {
        Some(provided_letters) => provided_letters,
        _ => "ABCDEFGHI",
    };

    let grid = Puzzle::new(letters).expect("Invalid letter selection");
    println!(
        "Created a grid with the following letters: {}",
        grid.letters()
    );

    // TODO this is just a test of the argument parser
    // let search = if let Some(letters) = letter_selection.letters.as_deref() {
    //     letters
    // } else {
    //     "ROBE"
    // };
    //
    // let found_words = trie.find_words(search).unwrap();
    // let found_missing_word = trie.contains("NONENSENOTAWORD");

    // assert!(!found_words.is_empty());
    // assert!(!found_missing_word);
    //
    // println!(
    //     "{} valid word(s) starting {search}: {:?}",
    //     found_words.len(),
    //     found_words
    // );
    //
    print!("Wordlist has {} bytes.", valid_words.len());
    print!("Wordlist has {} words.", count);
}
