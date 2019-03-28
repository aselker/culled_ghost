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
    println!("Loading file...");
    let mut file = File::open("wordlist-dedup.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let words = contents.split("\n");
    println!("Building trie...");
    // Build the trie
    let mut t = Trie { c: Default::default() };
    for word in words {
        t.insert_word(word);
    }
    println!("{}", t.pretty_print());
    println!("both: {:#?}", t.list_wins_losses());

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

    // If any move wins (i.e. forces an opponent to lose), then this returns that
    // move and any actions you might eventually have to take to maintain that win.
    // If no moves win (i.e. you are forced to lose), then this returns every action 
    // you might take, and all the things your opponent might have to do to maintain
    // their win.
    fn list_wins_losses(&self) -> (Vec<String>, Vec<String>) {
        // If this is empty, then it wins (kind of)
        if self.is_empty() {
            // return (vec![], vec![String::new()]);
            return (vec![String::new()], vec![]);
        }

        let mut losses_out = Vec::new();
        for (i, maybe) in self.c.iter().enumerate() {
            match maybe {
                None => {},
                Some(subtree) => {
                    let prefix = LETTERS[i].to_string();
                    let wins_and_losses = subtree.list_wins_losses();
                    let wins = wins_and_losses.0;
                    let losses = wins_and_losses.1;

                    // If this neither wins nor loses, then something is up...
                    if wins.len() == 0 && losses.len() == 0 {
                        panic!("{} neither wins nor loses!", prefix)
                    }

                    // If we can force them to lose, do it
                    if wins.len() == 0 {
                        let mut wins_out = Vec::new();
                        for loss in losses {
                            wins_out.push(prefix.clone() + &loss);
                        }
                        return (wins_out, vec![]);
                    }
                    
                    // Otherwise, add to the list of losses
                    for win in wins {
                        losses_out.push(prefix.clone() + &win);
                    }
                },
            }
        }

        // We lose.
        return (vec![], losses_out);
    }
} // end impl
