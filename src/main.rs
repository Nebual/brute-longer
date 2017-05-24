extern crate brutelonger;

use std::env;

use brutelonger::*;

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
    for line in brute_words_from_int(target, max, limit) {
        println!("{}", line);
    }
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

    let target = words_to_int(adj.to_owned(), noun.to_owned(), max);
    println!("{}", target);
}
