use std::{cmp::Ordering, str::FromStr};

fn main() {}

#[derive(Debug)]
struct Card {
    value: u8,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        Card {
            value: match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).unwrap() as u8,
            },
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().map(Card::from).collect::<Vec<_>>();
        Ok(Hand { cards })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hand() {
        let hand = "32T3K".parse::<Hand>().unwrap();

        assert_eq!(
            hand.cards,
            vec![
                Card { value: 3 },
                Card { value: 2 },
                Card { value: 10 },
                Card { value: 3 },
                Card { value: 13 },
            ]
        );
    }
}
