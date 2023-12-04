use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let max_val_hand = Hand {
        red: 12,
        blue: 14,
        green: 13,
    };
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
}

#[cfg(test)]
mod test {

    use super::*;

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
}
