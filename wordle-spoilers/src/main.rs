use std::collections::HashMap;
use wordle::{Score, POSSIBLE_WORDS};

/*
fn pass(lexicon: &[&'static str]) -> (&'static str, Vec<&'static str>) {
    let mut bests = std::collections::HashMap::<&'static str, Vec<&'static str>>::new();
    for word in lexicon.iter() {
        let mut result = std::collections::HashMap::<Score, Vec<&'static str>>::new();
        for guess in lexicon.iter() {
            let s = score(word, guess);
            result.entry(s).or_insert(vec![]).push(guess);
        }
        let best = result.iter().max_by_key(|(_, v)| v.len()).unwrap();
        println!("best for {} is {:?} ({})", word, best.0, best.1.len());
        bests.insert(word, best.1.clone());
    }
    let result = bests.iter().min_by_key(|(_, v)| v.len()).unwrap();
    println!("best overall: {} ({})", result.0, result.1.len());
    (result.0, result.1.clone())
}

fn main() {
    let mut guess = pass(&POSSIBLE_WORDS);
    while guess.1.len() > 1 {
        println!("{}: {}", guess.0, guess.1.len());
        guess = pass(&guess.1);
    }
    println!("{}", guess.0);
}
*/

fn main() {
    let mut results = HashMap::<Score, Vec<(&'static str, &'static str)>>::new();
    for word in POSSIBLE_WORDS {
        for guess in POSSIBLE_WORDS {
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
