#![deny(elided_lifetimes_in_paths)]
#![deny(missing_docs)]
#![deny(warnings)]

//! Solve a Squaredle puzzle using rust

use basic_trie::DatalessTrie;
use std::fs;

fn main() {
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
