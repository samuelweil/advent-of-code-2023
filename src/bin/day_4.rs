use advent_of_code_2023::{io::read_lines, regex};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let cards = read_lines("inputs/day_4.txt")
        .map(|line| Card::from(&line))
        .collect::<Vec<_>>();

    println!(
        "Day 4, Star 1: {}",
        cards
            .iter()
            .map(|card| { score(card.matches()) })
            .sum::<i32>()
    );

    let n_cards = total_scratchcards(&cards[..]);
    println!("Day 4, Star 2: {}", n_cards);
}

static CARD_REGEX: Lazy<Regex> = regex!(r"Card .+: (?<winning>.+) \| (?<mine>.+)");

struct Card {
    winning: Vec<i32>,
    mine: Vec<i32>,
}

impl Card {
    fn from(inp: &str) -> Self {
        let captures = CARD_REGEX.captures(inp).unwrap();
        let winning = unwrap_group(&captures, "winning");
        let mine = unwrap_group(&captures, "mine");

        Card {
            winning: parse_ints(winning),
            mine: parse_ints(mine),
        }
    }

    fn matches(&self) -> i32 {
        let mut matches = 0;

        for num in &self.mine {
            if self.winning.contains(num) {
                matches += 1;
            }
        }

        matches
    }
}

fn score(matches: i32) -> i32 {
    if matches <= 1 {
        matches
    } else {
        2 * score(matches - 1)
    }
}

fn total_scratchcards(cards: &[Card]) -> usize {
    let mut instances: Vec<usize> = vec![1; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let card_instances = instances[idx];
        let matches = card.matches();
        for i in 1..(matches + 1) {
            instances[idx + i as usize] += card_instances;
        }
    }

    instances.iter().sum::<usize>()
}

fn unwrap_group<'a>(captures: &'a regex::Captures, name: &str) -> &'a str {
    captures.name(name).unwrap().as_str()
}

fn parse_ints(inp: &str) -> Vec<i32> {
    inp.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_card() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(card.winning, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.mine, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_count_matches() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let matches = card.matches();

        assert_eq!(matches, 4);
    }

    #[test]
    fn test_score_of_zero_matches_is_zero() {
        assert_eq!(score(0), 0);
    }

    #[test]
    fn test_first_match_results_in_one_point() {
        assert_eq!(score(1), 1);
    }

    #[test]
    fn test_subsequent_matches_double_the_score() {
        assert_eq!(score(2), 2);
        assert_eq!(score(3), 4);
        assert_eq!(score(4), 8);
    }

    #[test]
    fn test_total_scratchcards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = input
            .split('\n')
            .map(|line| Card::from(line))
            .collect::<Vec<_>>();

        assert_eq!(total_scratchcards(&cards), 30);
    }
}
