#![deny(missing_docs)]
#![deny(warnings)]

use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    is_final: bool,
    child_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    // Create new node
    pub fn new(is_final: bool) -> TrieNode {
        TrieNode {
            is_final,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }

    pub fn insert_value(&mut self, c: char, is_final: bool) {
        self.child_nodes.insert(c, TrieNode::new(is_final));
    }
}

/// the actual True structure
#[derive(Debug)]
pub struct TrieStruct {
    root_node: TrieNode,
}

impl TrieStruct {
    /// Create a TrieStruct
    pub fn create() -> TrieStruct {
        TrieStruct {
            root_node: TrieNode::new_root(),
        }
    }

    /// Insert a string into the Trie
    pub fn insert(&mut self, string_val: String) {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();
        let mut last_match = 0;

        for letter_counter in 0..char_list.len() {
            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                last_match = letter_counter;
                break;
            }
            last_match = letter_counter + 1;
        }

        if last_match == char_list.len() {
            current_node.is_final = true;
        } else {
            for new_counter in last_match..char_list.len() {
                current_node.insert_value(char_list[new_counter], false);
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }
            current_node.is_final = true;
        }
    }

    /// Is a string in the Trie?
    pub fn find(&mut self, string_val: String) -> bool {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();

        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                return false;
            } else {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[counter])
                    .unwrap();
            }
        }
        return true;
    }
}
