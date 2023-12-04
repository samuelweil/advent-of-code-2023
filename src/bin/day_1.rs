use advent_of_code_2023::io::read_lines;

use regex::{Match, Regex};

const DIGIT_REGEX: &str = r"\d";
const DIGIT_NAME_REGEX: &str = r"one|two|three|four|five|six|seven|eight|nine";

fn main() {
    let file_name = format!("inputs/day_{}.txt", 1);
    run(read_lines(&file_name));
}

pub fn run<'a, T>(inputs: T)
where
    T: Iterator<Item = String>,
{
    let parser_1 = SimpleCalibrationParser::new();
    let parser_2 = AdvancedCalibrationParser::new();

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

fn parse_digit<'a, T: Into<&'a str>>(m: T) -> Option<u32> {
    match m.into() {
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

struct SimpleCalibrationParser(Regex);

impl SimpleCalibrationParser {
    fn new() -> SimpleCalibrationParser {
        let re = Regex::new(DIGIT_REGEX).unwrap();
        SimpleCalibrationParser(re)
    }

    pub fn parse_line(&self, line: &str) -> Option<i32> {
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

struct AdvancedCalibrationParser {
    regex: Regex,
    rev_regex: Regex,
}

fn reversed_str<'a, T: Into<&'a str>>(s: T) -> String {
    s.into().chars().rev().collect::<String>()
}

impl AdvancedCalibrationParser {
    fn new() -> AdvancedCalibrationParser {
        let regex_str = format!(r"\d|{}", DIGIT_NAME_REGEX);

        // We reverse the regex so that we can find the last instance.
        let reverse_name_regex_str = reversed_str(DIGIT_NAME_REGEX);
        // Single digit detection doesn't need to be reversed
        let reverse_name_str = format!(r"\d|{}", &reverse_name_regex_str);

        AdvancedCalibrationParser {
            regex: Regex::new(&regex_str).unwrap(),
            rev_regex: Regex::new(&reverse_name_str).unwrap(),
        }
    }

    pub fn parse_line(&self, line: &str) -> Option<i32> {
        let first_match = self.regex.find(line)?;

        let reverse_line = reversed_str(line);
        let last_match = self.rev_regex.find(&reverse_line)?;

        let first_digit = parse_digit(first_match.as_str())?;
        let last_digit = parse_digit(&*reversed_str(last_match))?;

        let config_value = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .map_or(None, Some)?;

        Some(config_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn simplecalibrationparser_parses_digits() {
        let parser = SimpleCalibrationParser::new();
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
    fn advancedcalibration_parser_parses_digit_names() {
        let parser = AdvancedCalibrationParser::new();
        let result = INPUT_2
            .lines()
            .map(|s| parser.parse_line(s))
            .filter_map(|s| s)
            .collect::<Vec<i32>>();

        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], result);
        assert_eq!(281, result.iter().sum());
    }

    #[test]
    fn advancedcalibration_parser_parses_digit_chars() {
        let parser = AdvancedCalibrationParser::new();
        let result = INPUT_1
            .lines()
            .map(|s| parser.parse_line(s))
            .filter_map(|s| s)
            .collect::<Vec<i32>>();

        assert_eq!(vec![12, 38, 15, 77], result);
    }

    #[test]
    fn advancedcalibration_parser_parses_digit_names_with_overlap() {
        let parser = AdvancedCalibrationParser::new();
        let result = parser.parse_line("28gtbkszmrtmnineoneightmx");

        assert_eq!(result, Some(28));
    }
}
