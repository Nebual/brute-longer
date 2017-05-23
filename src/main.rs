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
macro_rules! print_return {
    ($x:expr) => {
        println!($x);
        return;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&("-h".to_owned())) {
        print_return!("Usage:\
            \n    brute-longer target max [limit]\
            \n    brute-longer -d adjective noun max");
    }
    if args.contains(&("-d".to_owned())) {
        main_decode_words(args);
    } else {
        main_brute_words(args);
    }
}

fn main_brute_words(args: Vec<String>) {

    if args.len() < 3 {
        print_return!("Usage: brute-longer target max [limit]");
    }
    let target: u64 = match args[1].parse() {
        Err(_) => {
            print_return!("Usage: brute-longer target max [limit]");
        },
        Ok(ok) => ok,
    };
    let max: u64 = match args[2].parse() {
        Err(_) => {
            print_return!("Usage: brute-longer target max [limit]");
        },
        Ok(ok) => ok,
    };
    if target >= max {
        print_return!("target must be < max!");
    }
    let mut limit: u8 = 1;
    if args.len() >= 4 {
        limit = match args[3].parse() {
            Err(_) => {
                print_return!("Usage: brute-longer target max [limit]");
            },
            Ok(ok) => ok,
        }
    }
    brute_words_from_int(target, max, limit);
}


fn main_decode_words(args: Vec<String>) {
    if args.len() < 5 {
        print_return!("Usage: brute-longer -d adjective noun max");
    }
    let ref adj = args[2];
    let ref noun = args[3];
    let max: u64 = match args[4].parse() {
        Err(_) => {
            print_return!("Usage: brute-longer -d adjective noun max");
        },
        Ok(ok) => ok,
    };

    let mut hasher = FnvHasher::default();
    adj.hash(&mut hasher);
    noun.hash(&mut hasher);
    let hash = hasher.finish();
    let target = hash % max;
    println!("{}", target);
}

fn brute_words_from_int(target: u64, max: u64, mut limit: u8) {
    let mut adjs = get_file_lines_iter("words/adjectives.txt");
    let mut nouns = get_file_lines_iter("words/nouns.txt");

    loop {
        let adj : std::string::String = match adjs.next() {
            None => {
                adjs = get_file_lines_iter("words/adjectives.txt");
                continue;
            },
            Some(x) => match x {
                Err(why) => panic!("adj failed?? {}", why.description()),
                Ok(y) => y,
            },
        };
        let noun : std::string::String = match nouns.next() {
            None => {
                nouns = get_file_lines_iter("words/nouns.txt");
                continue;
            }
            Some(x) => match x {
                Err(why) => panic!("noun failed?? {}", why.description()),
                Ok(y) => y,
            },
        };
        let mut hasher = FnvHasher::default();
        adj.hash(&mut hasher);
        noun.hash(&mut hasher);

        let hash = hasher.finish();
        if (hash % max) == target {
            println!("{} {}", adj, noun);
            if limit > 1 {
                limit -= 1;
            } else {
                break;
            }
        }

    }
}
