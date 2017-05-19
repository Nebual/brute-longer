extern crate fnv;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
use std::env;
use std::hash::Hash;
use std::hash::Hasher;
use fnv::FnvHasher;

fn get_file_lines_iter(name: &str) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let adj_file = match File::open(name) {
        Err(why) => panic!("couldn't open {}: {}", name, why.description()),
        Ok(file) => file,
    };
    let adj_reader = BufReader::new(adj_file);
    return adj_reader.lines();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: brute-longer [target] [max]");
        return;
    }
    let target: u64 = match args[1].parse() {
        Err(_) => {
            println!("Usage: brute-longer [target] [max]");
            return;
        },
        Ok(target) => target,
    };
    let max: u64 = match args[2].parse() {
        Err(_) => {
            println!("Usage: brute-longer [target] [max]");
            return;
        },
        Ok(max) => max,
    };
    if target >= max {
        println!("target must be < max!");
        return;
    }

    let mut adjs = get_file_lines_iter("words/adjectives.txt");
    let mut nouns = get_file_lines_iter("words/nouns.txt");

    let mut hasher = FnvHasher::default();
    let mut num_hashed = 0;
    loop {
        num_hashed += 1;
        let adj = match adjs.next() {
            None => {
                adjs = get_file_lines_iter("words/adjectives.txt");
                continue;
            },
            Some(x) => match x {
                Err(why) => panic!("adj failed?? {}", why.description()),
                Ok(y) => y,
            },
        };
        let noun = match nouns.next() {
            None => {
                nouns = get_file_lines_iter("words/nouns.txt");
                continue;
            }
            Some(x) => match x {
                Err(why) => panic!("noun failed?? {}", why.description()),
                Ok(y) => y,
            },
        };
        adj.hash(&mut hasher);
        noun.hash(&mut hasher);

        let hash = hasher.finish();
        if (hash % max) == target {
            println!("Found: {} {} after {} attempts", adj, noun, num_hashed);
            break;
        }

    }
}
