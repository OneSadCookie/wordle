use std::collections::HashMap;
use wordle::{Score, SETTABLE_WORDS};

fn main() {
    let mut results = HashMap::<Score, Vec<(&'static str, &'static str)>>::new();
    for word in SETTABLE_WORDS {
        for guess in SETTABLE_WORDS {
            let s = wordle::score(word, guess);
            results.entry(s).or_insert(vec![]).push((word, guess));
        }
    }
    for (k, v) in results {
        println!("{:?}: {}", k, v.len());
        if v.len() < 10 {
            println!("{:?}", v);
        }
    }
}
