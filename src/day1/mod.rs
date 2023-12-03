pub fn run(inputs: String) {}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn read_digit(line: &str, at_index: usize) -> u32 {
    line.chars()
        .nth(at_index)
        .map(|c| c.to_digit(10).unwrap())
        .unwrap()
}

fn calibration(doc: &str) -> Vec<i32> {
    let mut result = Vec::new();

    for line in doc.lines() {
        let get_int = |index: usize| read_digit(line, index);

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
        let result = super::calibration(INPUT);

        assert_eq!(vec![12, 38, 15, 77], result);
    }
}
