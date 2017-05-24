extern crate fnv;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
use std::hash::Hash;
use std::hash::Hasher;
use std::string::String;
use fnv::FnvHasher;

fn get_file_lines_iter(name: &str) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let adj_file = match File::open(name) {
        Err(why) => panic!("couldn't open {}: {}", name, why.description()),
        Ok(file) => file,
    };
    let adj_reader = BufReader::new(adj_file);
    return adj_reader.lines();
}

pub fn brute_words_from_int(target: u64, max: u64, mut limit: u8) -> Vec<String> {
    let mut adjs = get_file_lines_iter("words/adjectives.txt");
    let mut nouns = get_file_lines_iter("words/nouns.txt");

    let mut results : Vec<String> = Vec::new();
    results.reserve_exact(limit as usize);
    loop {
        let adj : String = match adjs.next() {
            None => {
                adjs = get_file_lines_iter("words/adjectives.txt");
                continue;
            },
            Some(x) => match x {
                Err(why) => panic!("adj failed?? {}", why.description()),
                Ok(y) => y,
            },
        };
        let noun : String = match nouns.next() {
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
            results.push(format!("{} {}", adj, noun));
            if limit > 1 {
                limit -= 1;
            } else {
                break;
            }
        }

    }
    return results;
}

pub fn words_to_int(adj: String, noun: String, max: u64) -> u64 {
    let mut hasher = FnvHasher::default();
    adj.hash(&mut hasher);
    noun.hash(&mut hasher);
    let hash = hasher.finish();
    let target = hash % max;
    return target;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_words_to_int() {
        let adj = "smooth";
        let noun = "flyover";
        let max = 100000;
        assert_eq!(108, words_to_int(adj.to_owned(), noun.to_owned(), max));
    }

    #[test]
    fn test_brute_words_from_int() {
        let mut words = brute_words_from_int(108, 1000, 2);
        assert_eq!(2, words.len());
        assert_eq!("onshore assignment", words.pop().unwrap());
        assert_eq!("agitated countershot", words.pop().unwrap());
    }
}
