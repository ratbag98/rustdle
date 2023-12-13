#![deny(elided_lifetimes_in_paths)]
#![deny(missing_docs)]
#![deny(warnings)]

//! Solve a Squaredle puzzle using rust

use rustdle::trie;
use std::fs;

fn main() {
    let valid_words = fs::read_to_string("./word_list.txt").expect("problem reading word list");

    let mut trie = trie::TrieStruct::create();

    let words = valid_words.lines();

    let mut count = 0;
    for word in words {
        trie.insert(word.to_string());
        count += 1;
    }

    println!("Is YELLOW a word: {}", trie.find("YELLOW".to_string()));
    println!(
        "Is NONENSENOTAWORD a word: {}",
        trie.find("NONENSENOTAWORD".to_string())
    );

    print!("Wordlist has {} bytes.", valid_words.len());
    print!("Wordlist has {} words.", count);
}
