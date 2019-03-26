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

    let mut t = Trie { c: Default::default() };
    for word in words {
        println!("{}", word);
        insert_word(&mut t, word);
    }
    println!("{:#?}", t);

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

    // TODO: Make this be the default printing method
    fn pretty_print(&self) -> String {
        self.pretty_print_helper(String::new())
    }

    fn pretty_print_helper(&self, prefix: String) -> String {
        
        let mut out = String::new();

        for (i, subtree) in self.c.iter().enumerate() {
            match subtree {
                None => {},
                Some(ref subtree) => {
                    let this_prefix = (prefix.clone() + &(LETTERS[i].to_string()));
                    if subtree.is_empty() {
                        out += &this_prefix;
                     } else {
                        out += &subtree.pretty_print_helper(this_prefix);
                    }
                },
            }
        }
        return out;
    }
} // end impl

fn insert_word(t: &mut Trie, w: &str) {
    if w.len() == 0 { return; } 

    let (heads, tail) = w.split_at(1);
    match heads.chars().next() { // It should be exactly one char...
        None => {},
        Some(head) =>  {
            let n = letter_index(head);
            match t.c[n] {
                None => {
                    let mut subtree = Trie { c: Default::default() };
                    insert_word(&mut subtree, tail);
                    t.c[n] = Some(Box::new(subtree));
                },
                Some(ref mut subtree) => {
                    insert_word(subtree, tail);
                },
            }
        },

    }

}
