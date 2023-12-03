pub fn run<'a, T: Iterator<Item = String>>(inputs: T) {
    let result = calibration(inputs).iter().sum::<i32>();
    println!("Day 1, Star 1: {}", result);
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn read_digit(line: &str, at_index: usize) -> u32 {
    line.chars()
        .nth(at_index)
        .map(|c| c.to_digit(10).unwrap())
        .unwrap()
}

fn calibration<'a, T: Iterator<Item = String>>(doc: T) -> Vec<i32> {
    let mut result = Vec::new();

    for line in doc {
        let get_int = |index: usize| read_digit(line.as_str(), index);

        let first_digit = line.find(is_digit).map(get_int).unwrap();
        let last_digit = line.rfind(is_digit).map(get_int).unwrap();
        let configuration = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .unwrap();
        result.push(configuration);
    }

    result
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn test_main() {
        let result = super::calibration(INPUT.lines().map(String::from));

        assert_eq!(vec![12, 38, 15, 77], result);
    }
}
