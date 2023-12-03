use regex::{Match, Regex};

pub fn run<'a, T: Iterator<Item = String>>(inputs: T) {
    let result = calibration(inputs).iter().sum::<i32>();
    println!("Day 1, Star 1: {}", result);
}

struct DigitFinder(Regex);

fn parse_digit(m: Match<'_>) -> Option<u32> {
    m.as_str().parse::<u32>().map_or(Option::None, Option::Some)
}

impl DigitFinder {
    fn new() -> DigitFinder {
        let re = Regex::new(r"\d").unwrap();
        DigitFinder(re)
    }

    fn digits(&self, line: &str) -> Option<(u32, u32)> {
        let matches: Vec<Match> = self.0.find_iter(line).collect();

        if matches.len() < 1 {
            return Option::None;
        }

        let first_digit = parse_digit(matches[0])?;
        let last_digit = parse_digit(matches[matches.len() - 1])?;

        Some((first_digit, last_digit))
    }
}

fn calibration<'a, T: Iterator<Item = String>>(doc: T) -> Vec<i32> {
    let mut result = Vec::new();
    let finder = DigitFinder::new();

    for line in doc {
        let (first_digit, last_digit) = finder.digits(&line).unwrap();
        let configuration = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .unwrap();
        result.push(configuration);
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn test_parsing_number() {
        let result = calibration(INPUT_1.lines().map(String::from));
        assert_eq!(vec![12, 38, 15, 77], result);
    }
}
