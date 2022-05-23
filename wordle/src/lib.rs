mod guessable;
mod settable;

pub use guessable::EXTRA_GUESSABLE_WORDS;
pub use settable::SETTABLE_WORDS;

trait IterCountMatching {
    type Item: PartialEq;
    fn count_matching(self, item: Self::Item) -> usize;
}

impl<T: Iterator> IterCountMatching for T
where
    T::Item: PartialEq,
{
    type Item = T::Item;
    fn count_matching(self, reference: Self::Item) -> usize {
        self.fold(
            0,
            |count, item| {
                if item == reference {
                    count + 1
                } else {
                    count
                }
            },
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Peg {
    None,
    Yellow,
    Green,
}

impl std::fmt::Debug for Peg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Peg::*;
        write!(
            f,
            "{}",
            match self {
                None => ".",
                Yellow => "Y",
                Green => "G",
            }
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Score {
    pub pegs: [Peg; 5],
}

impl Score {
    pub fn new() -> Score {
        Score {
            pegs: [Peg::None; 5],
        }
    }
}

impl std::fmt::Debug for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}",
            self.pegs[0], self.pegs[1], self.pegs[2], self.pegs[3], self.pegs[4]
        )
    }
}

pub fn score_bytes(word: &[u8; 5], guess: &[u8; 5]) -> Score {
    let mut score = Score::new();
    let mut cross_word = [(0u8, false); 5];
    for i in 0..5 {
        cross_word[i] = (word[i], false);
    }

    for i in 0..5 {
        if cross_word[i].0 == guess[i] && !cross_word[i].1 {
            score.pegs[i] = Peg::Green;
            cross_word[i].1 = true;
        }
    }

    for i in 0..5 {
        if score.pegs[i] == Peg::None {
            for j in 0..5 {
                if cross_word[j].0 == guess[i] && !cross_word[j].1 {
                    score.pegs[i] = Peg::Yellow;
                    cross_word[j].1 = true;
                    break;
                }
            }
        }
    }
    score
}

pub fn score(word: &str, guess: &str) -> Score {
    score_bytes(
        word.as_bytes().try_into().unwrap(),
        guess.as_bytes().try_into().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use crate::Peg::*;
    use crate::*;

    #[test]
    fn test_count_matching() {
        assert_eq!("ABCDE".as_bytes().iter().count_matching(&b'A'), 1);
        assert_eq!("ABCDE".as_bytes().iter().count_matching(&b'E'), 1);
        assert_eq!("ABCDA".as_bytes().iter().count_matching(&b'A'), 2);
        assert_eq!("AAAAA".as_bytes().iter().count_matching(&b'A'), 5);
    }

    #[test]
    fn test_score() {
        assert_eq!(
            score("ABCDE", "FGHIJ"),
            Score {
                pegs: [None, None, None, None, None]
            }
        );
        assert_eq!(
            score("ABCDE", "ABCDE"),
            Score {
                pegs: [Green, Green, Green, Green, Green],
            }
        );
        assert_eq!(
            score("ABCDE", "ABDEC"),
            Score {
                pegs: [Green, Green, Yellow, Yellow, Yellow],
            }
        );
        assert_eq!(
            score("ABCDE", "FABIJ"),
            Score {
                pegs: [None, Yellow, Yellow, None, None],
            }
        );
        assert_eq!(
            score("STATS", "TSARS"),
            Score {
                pegs: [Yellow, Yellow, Green, None, Green],
            }
        );
        assert_eq!(
            score("SWIRL", "SWILL"),
            Score {
                pegs: [Green, Green, Green, None, Green],
            }
        );
        assert_eq!(
            score("SWILL", "SWIRL"),
            Score {
                pegs: [Green, Green, Green, None, Green],
            }
        );
    }
}
