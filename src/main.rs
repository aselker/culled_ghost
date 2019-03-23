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
    let split = contents.split("\n");
    for s in split {
        println!("{}", s);
    }
    Ok(())
}

fn letter_index(c: char) -> usize {
    match (LETTERS.iter().position(|&r| r == c)) {
        Some (x) => x,
        None => 255,
    }
}

// type Trie = [Option<Box<Trie>>; 26];
struct Trie { 
    c: [Option<Box<Trie>>; 26],
}

fn is_empty(t: Trie) -> bool {
    t.c.iter().all(|x| x.is_none())
}

fn insert_word<'o>(o: &'o mut Option<Box<Trie>>, w: &[char]) {

    /*
    let mut t = match *o {
        None => Trie { c: Default::default() },
        Some(x) => *x,
    };
    */
    let mut t = Trie { c: Default::default() };
    if (*o).is_some() {
        t = *((*o).unwrap());
    }
    
    match w.split_first() {
        None => {},
        Some(wt) => {
            let i = letter_index(w[0]);
            insert_word(&mut (t.c[i]), wt.1)
        }
    }

    *o = Some(Box::new(t))

}
