use regex::{Match, Regex};

const CHAR_REGEX: &str = r"\d";
const ALL_REGEX: &str = r"\d|one|two|three|four|five|six|seven|eight|nine";

pub fn run<'a, T>(inputs: T)
where
    T: Iterator<Item = String>,
{
    let mut parser_1 = CalibrationParser::new(CHAR_REGEX);
    let mut parser_2 = CalibrationParser::new(ALL_REGEX);

    let mut calib_1 = 0;
    let mut calib_2 = 0;

    for line in inputs {
        if let Some(i1) = parser_1.parse_line(&line) {
            calib_1 += i1;
        }
        if let Some(i2) = parser_2.parse_line(&line) {
            calib_2 += i2;
        }
    }

    println!("Day 1, Star 1: {}", calib_1);
    println!("Day 1, Star 2: {}", calib_2);
}

struct CalibrationParser(Regex);

fn parse_digit(m: Match<'_>) -> Option<u32> {
    match m.as_str() {
        "one" => return Option::Some(1),
        "two" => return Option::Some(2),
        "three" => return Option::Some(3),
        "four" => return Option::Some(4),
        "five" => return Option::Some(5),
        "six" => return Option::Some(6),
        "seven" => return Option::Some(7),
        "eight" => return Option::Some(8),
        "nine" => return Option::Some(9),
        c => c.parse::<u32>().map_or(Option::None, Option::Some),
    }
}

impl CalibrationParser {
    fn new(pattern: &str) -> CalibrationParser {
        let re = Regex::new(pattern).unwrap();
        CalibrationParser(re)
    }

    pub fn parse_line(&mut self, line: &str) -> Option<i32> {
        let matches: Vec<Match> = self.0.find_iter(line).collect();

        if matches.len() < 1 {
            return Option::None;
        }

        let first_digit = parse_digit(matches[0])?;
        let last_digit = parse_digit(matches[matches.len() - 1])?;

        let config_value = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .map_or(None, Some)?;

        Some(config_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day1::CalibrationParser;

    const INPUT_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn test_parsing_number() {
        let mut parser = CalibrationParser::new(CHAR_REGEX);
        let result = INPUT_1
            .lines()
            .map(|s| parser.parse_line(s))
            .filter_map(|s| s)
            .collect::<Vec<i32>>();

        assert_eq!(vec![12, 38, 15, 77], result);
    }

    const INPUT_2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test_parsing_digits_by_name() {
        let mut parser = CalibrationParser::new(ALL_REGEX);
        let result = INPUT_2
            .lines()
            .map(|s| parser.parse_line(s))
            .filter_map(|s| s)
            .collect::<Vec<i32>>();

        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], result);
        assert_eq!(281, result.iter().sum());

        let result_2 = INPUT_1
            .lines()
            .map(|s| parser.parse_line(s))
            .filter_map(|s| s)
            .collect::<Vec<i32>>();

        assert_eq!(vec![12, 38, 15, 77], result_2);
    }
}
