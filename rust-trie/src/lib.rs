//! This is a standard implementation of a [trie](https://en.wikipedia.org/wiki/Trie) or prefix tree, data structure.
//!
//! No optimizations and is `O(n)` across all operations

use std::collections::VecDeque;
use std::fmt::{Display, Formatter};


#[derive(Default,Debug)]
struct Node {
    /// children of this Node
    children: Vec<Node>,

    /// the prefix character stored in this node
    key: Option<char>,

    /// the 'word' stored in this Node but only if this Node is a terminal(leaf) Node
    value: Option<String>,

    /// if true it indicates the node is a `terminal (leaf)` node, i.e. marks the end of a word
    terminal: bool,
}

impl Node {
    /// returns a new node, with all fields set to their default values
    fn new() -> Self {
        Node {
            ..Default::default()
        }
    }

    /// returns a new Node with its `key` field set to `Some(k)`
    fn with_key(k: char) -> Self {
        Node {
            key: Some(k),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node::new(),
        }
    }

    /// inserts `s` into the trie, overwriting any previously existing values
    pub fn insert(&mut self, s: &str) {
        let mut curr = &mut self.root;
        for ch in s.chars() {
            match curr.children.binary_search_by(|f| f.key.cmp(&Some(ch))) {
                Ok(idx) => {
                    // char was found
                    // set curr to child Node and continue the traversing the Trie
                    curr = &mut curr.children[idx];
                },
                Err(idx) => {
                    // char not found, insert new node with char
                    curr.children.insert(idx, Node::with_key(ch));
                    curr = &mut curr.children[idx];
                },
            }
        }
        // should be at a terminal node, set the node's value but only if it doesn't already exist
        if curr.terminal && curr.value == Some(s.to_string()) {
            return
        } else {
            curr.terminal = true;
            curr.value.replace(s.to_string());
        }

    }

    /// returns `true` if `s` exists within this trie, otherwise `false`
    pub fn exists(&self, s: &str) -> bool {
        let mut curr = &self.root;
        for c in s.chars() {
            match curr.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(idx) => {
                    curr = &curr.children[idx];
                },
                Err(_) => {
                    return false;
                }
            }
        }
        // check if we are at a terminal node and return true
        curr.terminal
    }

    /// returns any words in this trie that are equal to, or begin with `s`. If no words are found
    /// then an empty Vector is returned
    pub fn search(&self, s: &str) -> Vec<String> {
        if s.is_empty() {
            return vec![];
        }
        let mut curr = &self.root;
        for c in s.chars() {
            match curr.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(idx) => {
                    curr = &curr.children[idx];
                },
                Err(_) => {
                    return Vec::new();
                }
            }
        }
        // should be at end of the prefix match, need to Depth First Search and find all
        // matching nodes
        let mut matches = Vec::new();
        let mut queue = vec![curr];
        while let Some(n) = queue.pop() {
            // add all of curr nodes' children to the queue
            n.children.iter().for_each(|cn| queue.push(cn));

            if n.terminal {
                let value = n.value.as_ref().unwrap();
                matches.push(value.to_owned());
            }
        }
        // sort matches
        matches.sort_by(|n1, n2| n2.cmp(&n1));
        matches
    }


    /// deletes `s` from the trie.
    /// returns `true` if `s` was deleted, else `false` if `s` was not found in the trie
    pub fn delete(&mut self, s: &str) -> bool {
        // this is a basic delete operation in that it only decrements the terminal node count, and
        // does actually remove the trie's internal nodes.
        let mut curr = &mut self.root;
        for c in s.chars() {
            match curr.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(idx) => {
                    curr = &mut curr.children[idx];
                },
                Err(_) => {
                    return false;
                }
            }
        }
        // check if we are at a terminal node and decrement its count
        if curr.terminal {
            return match &curr.value {
                Some(val) if val == s => {
                    curr.terminal = false;
                    curr.value.take();
                    true
                },
                _ => {
                    false
                }
            }
        } else {
            // word was already deleted or never existed in the trie
            false
        }
    }
}

impl Display for Trie {
    /// Display prints the keys of this trie in **level order**.
    /// Along with the key, the Node.count will be printed in parentheses
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // display the trie using a level traversal
        let mut queue: VecDeque<&Node> = VecDeque::new();
        let root = &self.root;
        queue.push_back(root);

        while !queue.is_empty() {
            for _ in 0..queue.len() {
               if let Some(node) = queue.pop_front() {
                   for c in node.children.iter() {
                       write!(f, "{}({}) ", &c.key.unwrap(), &c.terminal)?;
                       if !c.children.is_empty() {
                           queue.push_back(c);
                       }
                   }
               }
            }
            if !queue.is_empty() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crate::Trie;

    // returns a new trie with some default values
    fn new_trie() -> Trie {
        let mut trie = Trie::new();
        trie.insert("a");
        trie.insert("to");
        trie.insert("tea");
        trie.insert("apples");
        trie.insert("an");
        trie.insert("test");
        trie.insert("tea");
        trie.insert("anna");
        trie.insert("annabelle");
        trie
    }

    #[test]
    fn display_trie() {
        let trie = new_trie();
        println!("{}", trie);
    }

    #[test]
    fn exists_finds_existing_string() {
        let trie = new_trie();
        assert!(trie.exists("tea"));
    }

    #[test]
    fn exists_returns_false_for_empty_trie() {
        let trie = new_trie();
        assert_eq!(trie.exists("testing"), false);
    }

    #[test]
    fn string_exists() {
        let trie = new_trie();
        assert!(trie.exists("a"));
    }

    #[test]
    fn search_returns_three_words() {
        let trie = new_trie();
        let res = trie.search("an");
        assert_eq!(res.len(), 3);
        assert!(res.contains(&"an".to_string()));
        assert!(res.contains(&"anna".to_string()));
        assert!(res.contains(&"annabelle".to_string()));
    }

    #[test]
    fn search_returns_empty_vec() {
        let trie = new_trie();
        let res = trie.search("zebra");
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn search_with_empty_string_returns_false() {
        let trie = new_trie();
        let res = trie.search("");
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn should_delete() {
        let mut trie = Trie::new();
        trie.insert("tab");
        trie.insert("teb");
        trie.insert("tec");
        trie.delete("teb");

        assert_eq!(trie.exists("teb"), false)
    }
}
