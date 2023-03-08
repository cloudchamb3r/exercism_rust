enum CardShape {
    Heart, 
    Diamond, 
    Club, 
    Spade,
}

struct Card {
    value: usize, 
    shape: CardShape,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Card {
    fn new(s: &str) -> Card {
        let value = *&s[..s.len() - 1]
            .chars()
            .rev()
            .skip(1)
            .into_iter()
            .fold(0, |acc, cur| {
                if cur == 'A' {
                    14
                } else if cur == 'K' {
                    13
                } else if cur == 'Q' {
                    12
                } else if cur == 'J' {
                    11
                } else {
                    10 * acc + (cur as usize - '0' as usize)
                }
            });
            
        let shape = match  s
            .chars().rev().nth(0).unwrap() {
            'H' => Ok(CardShape::Heart), 
            'D' => Ok(CardShape::Diamond),
            'C' => Ok(CardShape::Club),
            'S' => Ok(CardShape::Spade),
            _ => Err("invalid shape"),
        }.unwrap();

        Card {
            value, 
            shape   
        }
    }
}



/// ---

use std::cmp::Ordering::*;
enum CardFamily {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    NoPair,
}

impl PartialEq for CardFamily {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for CardFamily {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            CardFamily::StraightFlush => {
                match other {
                    CardFamily::StraightFlush => Some(Equal),
                    _ => Some(Greater),
                }
            },
            CardFamily::FourOfAKind => {
                match other {
                    CardFamily::FourOfAKind => Some(Equal),
                    CardFamily::StraightFlush => Some(Less),
                    _ => Some(Greater),
                }
            },
            CardFamily::FullHouse => {
                match other {
                    CardFamily::FullHouse => Some(Equal),
                    CardFamily::StraightFlush | CardFamily::FourOfAKind => Some(Less),
                    _ => Some(Greater),
                }
            },
            CardFamily::Flush => {
                match other {
                    CardFamily::Flush => Some(Equal),
                    CardFamily::StraightFlush | CardFamily::FourOfAKind | CardFamily::FullHouse => Some(Less),
                    _ => Some(Less),
                }
            },
            CardFamily::Straight => {
                match other {
                    CardFamily::Straight => Some(Equal),
                    CardFamily::FourOfAKind | CardFamily::FullHouse | CardFamily::Flush => Some(Less),
                    _ => Some(Greater),
                }
            },
            CardFamily::ThreeOfAKind => {
                match other {
                    CardFamily::ThreeOfAKind => Some(Equal),
                    CardFamily::TwoPair | CardFamily::OnePair | CardFamily::NoPair => Some(Greater),
                    _ => Some(Less),
                }
            },
            CardFamily::TwoPair => {
                match other {
                    CardFamily::TwoPair => Some(Equal),
                    CardFamily::OnePair | CardFamily::NoPair => Some(Greater),
                    _ => Some(Less)
                }
            },
            CardFamily::OnePair => {
                match other {
                    CardFamily::OnePair => Some(Equal),
                    CardFamily::NoPair => Some(Greater),
                    _ => Some(Less),
                }
            },
            CardFamily::NoPair => {
                match other {
                    CardFamily::NoPair => Some(Equal),
                    _ => Some(Less),
                }
            },
        }
    }
}

fn get_card_familiy(hand : &[Card])  -> (CardFamily, usize) {

    (CardFamily::Straight, 0)
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {

    vec![]
}
