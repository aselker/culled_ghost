#![deny(warnings)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::prelude::*;
use std::boxed::Box;

const LETTERS: [char; 26] =['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

fn main() -> std::io::Result<()> {
    // First, load the word list (which is already thinned)
    let mut file = File::open("wordlist-short.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let words = contents.split("\n");

    // Build the trie
    let mut t = Trie { c: Default::default() };
    for word in words {
        t.insert_word(word);
    }
    println!("{}", t.pretty_print());
    println!("{}", t.will_win());
    println!("{}", t.will_lose());

    Ok(())
}

fn letter_index(c: char) -> usize {
    match (LETTERS.iter().position(|&r| r == c)) {
        Some (x) => x,
        None => {
            panic!("Character {} is not in the alphabet", c);
        },
    }
}

#[derive(Debug)]
struct Trie { 
    c: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn is_empty(&self) -> bool {
        self.c.iter().all(|x| x.is_none())
    }

    // All words represented by the Trie, newline-separated
    // TODO: Make this be the default printing method
    fn pretty_print(&self) -> String {
        self.pretty_print_helper(String::new())
    }

    // pretty_print with a prefix
    fn pretty_print_helper(&self, prefix: String) -> String {
        
        let mut out = String::new();

        for (i, subtree) in self.c.iter().enumerate() {
            match subtree {
                None => {},
                Some(ref subtree) => {
                    let this_prefix = (prefix.clone() + &(LETTERS[i].to_string()));
                    if subtree.is_empty() {
                        out += &this_prefix;
                        out += "\n";
                     } else {
                        out += &subtree.pretty_print_helper(this_prefix);
                    }
                },
            }
        }
        return out;
    }

    fn insert_word(&mut self, w: &str) {
        if w.len() == 0 { return; } 

        let (heads, tail) = w.split_at(1);
        match heads.chars().next() { // It should be exactly one char...
            None => {},
            Some(head) =>  {
                let n = letter_index(head);
                match self.c[n] {
                    None => {
                        let mut subtree = Trie { c: Default::default() };
                        subtree.insert_word(tail);
                        self.c[n] = Some(Box::new(subtree));
                    },
                    Some(ref mut subtree) => {
                        subtree.insert_word(tail);
                    },
                }
            },

        }

    }

    // Don't use, it wastes effort
    fn will_win(&self) -> bool {
        self.c.iter().all(|x| match x {
            None => true,
            Some(ref subtree) => subtree.will_lose(),
        })
    }

    fn will_lose(&self) -> bool {
        self.c.iter().any(|x| match x {
            None => false,
            Some(ref subtree) => subtree.will_win(),
        })
    }

    fn cull(&mut self) {
        for (i, maybe) in self.c.iter().enumerate() {
            match maybe {
                None => {},
                Some(ref mut subtree) => {
                    subtree.cull();
                    if subtree.is_empty() {
                        self.c[i] = None;
                    }
                },
            }
        }
    }
} // end impl
