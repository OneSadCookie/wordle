use wordle::{Score, SETTABLE_WORDS};

fn pass(lexicon: &[&'static str]) -> (&'static str, Vec<&'static str>) {
    let mut bests = std::collections::HashMap::<&'static str, Vec<&'static str>>::new();
    for word in lexicon.iter() {
        let mut result = std::collections::HashMap::<Score, Vec<&'static str>>::new();
        for guess in lexicon.iter() {
            let s = wordle::score(word, guess);
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
    let mut guess = pass(&SETTABLE_WORDS);
    while guess.1.len() > 1 {
        println!("{}: {}", guess.0, guess.1.len());
        guess = pass(&guess.1);
    }
    println!("{}", guess.0);
}
