use advent_of_code_2023::io::read_lines;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    for (line_no, line) in read_lines("data/day_3.txt").enumerate() {
        let (p, s) = parse_schematic(line_no, &line);
        parts.extend(p);
        symbols.extend(s);
    }
}

#[derive(PartialEq, Debug)]
struct PartNumber {
    value: i32,
    row: usize,
    start: usize,
    end: usize,
}

#[derive(PartialEq, Debug)]
struct Symbol {
    row: usize,
    column: usize,
}

static PART_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d\.]").unwrap());

fn parse_schematic(line_no: usize, line: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let parts = PART_REGEX
        .find_iter(line)
        .map(|m| {
            let number = m.as_str().parse::<i32>().expect("Failed to parse number");
            let start = m.start();
            let end = m.end();

            PartNumber {
                value: number,
                row: line_no,
                start,
                end,
            }
        })
        .collect();

    let symbols = SYMBOL_REGEX
        .find_iter(line)
        .map(|m| Symbol {
            row: line_no,
            column: m.start(),
        })
        .collect();

    (parts, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_parts() {
        let (parts, symbols) = parse_schematic(1, "467..114..");
        assert_eq!(
            parts,
            vec![
                PartNumber {
                    value: 467,
                    row: 1,
                    start: 0,
                    end: 3,
                },
                PartNumber {
                    value: 114,
                    row: 1,
                    start: 5,
                    end: 8,
                }
            ]
        );
        assert_eq!(symbols.len(), 0);
    }

    #[test]
    fn test_parse_symbols() {
        let (parts, symbols) = parse_schematic(3, "...$.*....");

        assert_eq!(parts.len(), 0);
        assert_eq!(
            symbols,
            vec![Symbol { row: 3, column: 3 }, Symbol { row: 3, column: 5 }]
        );
    }

    #[test]
    fn test_parse_mixed() {
        let (parts, symbols) = parse_schematic(8, "617*......");
        assert_eq!(
            parts,
            vec![PartNumber {
                value: 617,
                row: 8,
                start: 0,
                end: 3,
            }]
        );
        assert_eq!(symbols, vec![Symbol { row: 8, column: 3 }])
    }
}
