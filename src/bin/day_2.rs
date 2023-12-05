use advent_of_code_2023::io::read_lines;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let max_val_hand = Hand {
        red: 12,
        blue: 14,
        green: 13,
    };

    let mut sum_valid_game_ids = 0;
    let mut sum_powers = 0;

    for line in read_lines("inputs/day_2.txt") {
        let game = Game::from(&line);
        if game.is_valid(&max_val_hand) {
            sum_valid_game_ids += game.id;
        }

        sum_powers += game.min_val_hand().power();
    }

    println!("Day 2, Star 1: {}", sum_valid_game_ids);
    println!("Day 2, Star 2: {}", sum_powers);
}

#[derive(PartialEq, Debug)]
struct Hand {
    red: i32,
    blue: i32,
    green: i32,
}

static HAND_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<val>\d+) (?<color>red|blue|green)").unwrap());

impl Hand {
    fn from(inp: &str) -> Hand {
        let fail_msg = format!("{} is not a valid hand", inp);

        let mut set = Hand {
            red: 0,
            blue: 0,
            green: 0,
        };
        for cap in HAND_REGEX.captures_iter(inp) {
            let value = cap
                .name("val")
                .expect(&fail_msg)
                .as_str()
                .parse::<i32>()
                .expect(&fail_msg);
            let color = cap.name("color").expect(&fail_msg).as_str();

            match color {
                "red" => set.red = value,
                "blue" => set.blue = value,
                "green" => set.green = value,
                _ => panic!("Unknown color"),
            }
        }
        set
    }

    fn power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

struct Game {
    id: i32,
    hands: Vec<Hand>,
}

static GAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Game (?<id>\d+): (?<hands>.*)").unwrap());

impl Game {
    fn from(inp: &str) -> Game {
        let fail_msg = format!("{} is not a valid game", inp);

        let captures = GAME_REGEX.captures(inp).expect(&fail_msg);
        let id = captures
            .name("id")
            .expect(&fail_msg)
            .as_str()
            .parse::<i32>()
            .expect(&fail_msg);

        let hands = captures
            .name("hands")
            .expect(&fail_msg)
            .as_str()
            .split(";")
            .map(Hand::from)
            .collect();

        Game { id, hands }
    }

    fn is_valid(&self, max_hand: &Hand) -> bool {
        for hand in &self.hands {
            if hand.red > max_hand.red || hand.blue > max_hand.blue || hand.green > max_hand.green {
                return false;
            }
        }
        true
    }

    fn min_val_hand(&self) -> Hand {
        let mut min_hand = Hand {
            red: 0,
            blue: 0,
            green: 0,
        };

        for hand in &self.hands {
            if hand.red > min_hand.red {
                min_hand.red = hand.red;
            }
            if hand.blue > min_hand.blue {
                min_hand.blue = hand.blue;
            }
            if hand.green > min_hand.green {
                min_hand.green = hand.green;
            }
        }

        min_hand
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn valid_games<T: Iterator<Item = U>, U: AsRef<str>>(
        lines: T,
        max_val_hand: &Hand,
    ) -> Vec<i32> {
        lines
            .filter_map(|line| {
                let game = Game::from(line.as_ref());
                if game.is_valid(max_val_hand) {
                    Some(game.id)
                } else {
                    None
                }
            })
            .collect()
    }

    fn powers<T: Iterator<Item = U>, U: AsRef<str>>(lines: T) -> Vec<i32> {
        lines
            .map(|line| {
                let game = Game::from(line.as_ref());
                game.min_val_hand().power()
            })
            .collect()
    }

    #[test]
    fn test_parsing_game_id() {
        let result = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(result.id, 1);
    }

    #[test]
    fn test_parsing_game_hands() {
        let result = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            result.hands,
            vec![
                Hand {
                    red: 4,
                    blue: 3,
                    green: 0
                },
                Hand {
                    red: 1,
                    blue: 6,
                    green: 2
                },
                Hand {
                    red: 0,
                    blue: 0,
                    green: 2
                }
            ]
        );
    }

    #[test]
    fn test_parsing_set() {
        let result = Hand::from("3 blue, 4 red");
        assert_eq!(
            result,
            Hand {
                red: 4,
                blue: 3,
                green: 0
            }
        );
    }

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_valid_games() {
        let max_val_hand = Hand {
            red: 12,
            blue: 14,
            green: 13,
        };

        let result = valid_games(TEST_INPUT.lines(), &max_val_hand);
        assert_eq!(vec![1, 2, 5], result);
    }

    #[test]
    fn test_lowest_hand() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let result = game.min_val_hand();
        assert_eq!(
            result,
            Hand {
                red: 4,
                blue: 6,
                green: 2
            }
        );
    }

    #[test]
    fn test_power_of_hand() {
        let hand = Hand {
            red: 4,
            blue: 6,
            green: 2,
        };
        let result = hand.power();
        assert_eq!(result, 48);
    }

    #[test]
    fn test_powers() {
        let result = powers(TEST_INPUT.lines());
        assert_eq!(vec![48, 12, 1560, 630, 36], result);
    }
}
