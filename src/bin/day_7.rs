use std::{cmp::Ordering, collections::BTreeMap, str::FromStr};

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
    cards: [Card; 5],
    rank: HandRank,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s.chars().map(Card::from).collect::<Vec<_>>().try_into().unwrap();
        let rank = HandRank::from(&cards[..]);
        Ok(Hand { cards, rank })
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank_ordering = self.rank.partial_cmp(&other.rank)?;
        if rank_ordering == Ordering::Equal {
            for i in 0..5 {
                let card_ordering = self.cards[i].partial_cmp(&other.cards[i])?;
                if card_ordering != Ordering::Equal {
                    return Some(card_ordering);
                }
            }
            // This should be impossible
            None
        } else {
            Some(rank_ordering)
        }
    }
}

#[derive(Debug)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandRank {
    fn val(&self) -> u8 {
        match self {
            HandRank::HighCard => 1,
            HandRank::OnePair => 2,
            HandRank::TwoPair => 3,
            HandRank::ThreeOfAKind => 4,
            HandRank::FullHouse => 5,
            HandRank::FourOfAKind => 6,
            HandRank::FiveOfAKind => 7,
        }
    }
}

impl From<&[Card]> for HandRank {
    fn from(cards: &[Card]) -> Self {
        let mut cardinality = BTreeMap::new();
        for card in cards {
            *cardinality.entry(card.value).or_insert(0) += 1;
        }

        match cardinality.len() {
            1 => HandRank::FiveOfAKind,
            2 => {
                if cardinality.values().any(|&v| v == 4 || v == 1) {
                    HandRank::FourOfAKind
                } else {
                    HandRank::FullHouse
                }
            }
            3 => {
                if cardinality.values().any(|&v| v == 3) {
                    HandRank::ThreeOfAKind
                } else {
                    HandRank::TwoPair
                }
            }
            _ => {
                if cardinality.values().any(|&v| v == 2) {
                    HandRank::OnePair
                } else {
                    HandRank::HighCard
                }
            }
        }
    }
}

impl From<&[Card; 5]> for HandRank {
    fn from(cards: &[Card; 5]) -> Self {
        HandRank::from(&cards[..])
    }
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        self.val() == other.val()
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val().partial_cmp(&other.val())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hand() {
        let hand = Hand::from("32T3K");

        assert_eq!(
            hand.cards,
            [
                Card { value: 3 },
                Card { value: 2 },
                Card { value: 10 },
                Card { value: 3 },
                Card { value: 13 },
            ]
        );
    }

    struct HandRankTest {
        hand: &'static str,
        rank: HandRank,
    }

    impl HandRankTest {
        fn execute(&self) {
            let hand = Hand::from(self.hand);
            assert_eq!(HandRank::from(&hand.cards), self.rank);
        }
    }

    #[test]
    fn test_hand_rank() {
        let tests = vec![
            HandRankTest {
                hand: "33333",
                rank: HandRank::FiveOfAKind,
            },
            HandRankTest {
                hand: "3333K",
                rank: HandRank::FourOfAKind,
            },
            HandRankTest {
                hand: "333KK",
                rank: HandRank::FullHouse,
            },
            HandRankTest {
                hand: "3332K",
                rank: HandRank::ThreeOfAKind,
            },
            HandRankTest {
                hand: "3322K",
                rank: HandRank::TwoPair,
            },
            HandRankTest {
                hand: "32T3K",
                rank: HandRank::OnePair,
            },
            HandRankTest {
                hand: "AKQJ9",
                rank: HandRank::HighCard,
            },
        ];

        for test in tests {
            test.execute();
        }
    }

    #[test]
    fn test_hand_strength() {
        let full_house = Hand::from("333KK");
        let one_pair = Hand::from("32T3K");

        assert!(one_pair < full_house, "Hands of a lower rank should have a lower strenght");

        let lower_full_house = Hand::from("222KK");
        assert!(lower_full_house < full_house, "When hands have the same rank the lower value should have a lower strength");

        let full_house_2 = Hand::from("3K3K3");
        assert!(full_house < full_house_2, "First higher card determines strenght when rank is equal");
    }
}