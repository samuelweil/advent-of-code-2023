use std::cmp::max;

use advent_of_code_2023::io::read_lines;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let schematic = Schematic::parse(read_lines("day_3.txt"));

    {
        let result = schematic
            .valid_parts()
            .iter()
            .map(|p| p.number)
            .sum::<i32>();
        println!("Day 3, Star 1: {}", result);
    }
}

struct Schematic {
    raw_parts: Vec<Part>,
    raw_symbols: Vec<Symbol>,
}

impl Schematic {
    pub fn parse<T: Iterator<Item = U>, U: AsRef<str>>(lines: T) -> Self {
        let mut raw_parts = Vec::new();
        let mut raw_symbols = Vec::new();

        for (line_no, line) in lines.enumerate() {
            let (p, s) = Self::parse_line(line_no, line.as_ref().trim());
            raw_parts.extend(p);
            raw_symbols.extend(s);
        }

        Schematic {
            raw_parts,
            raw_symbols,
        }
    }

    pub fn valid_parts(&self) -> Vec<Part> {
        let parts = &self.raw_parts[..];
        parts
            .into_iter()
            .filter_map(|p| {
                if Self::is_adjacent(p, &self.raw_symbols) {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn parse_line(line_no: usize, line: &str) -> (Vec<Part>, Vec<Symbol>) {
        let parts = PART_REGEX
            .find_iter(line)
            .map(|m| {
                let number = m.as_str().parse::<i32>().expect("Failed to parse number");
                let start = m.start();
                let end = m.end();

                Part {
                    number,
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

    fn is_adjacent(part: &Part, symbols: &[Symbol]) -> bool {
        symbols.into_iter().any(|s| part.is_adjacent(s))
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Part {
    number: i32,
    row: usize,
    start: usize,
    end: usize,
}

impl Part {
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        let start_col = max(self.start, 1) - 1;
        let end_col = self.end; // Match goes 1 past the end

        let start_row = max(self.row, 1) - 1;
        let end_row = self.row + 1;

        let vertical_adjacent = symbol.row <= end_row && symbol.row >= start_row;
        let horizontal_adjacent = symbol.column <= end_col && symbol.column >= start_col;

        vertical_adjacent && horizontal_adjacent
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Symbol {
    row: usize,
    column: usize,
}

static PART_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d\.]").unwrap());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let schematic = Schematic::parse(input.lines());
        let part_nos = schematic
            .valid_parts()
            .iter()
            .map(|p| p.number)
            .collect::<Vec<_>>();

        assert_eq!(part_nos, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn test_parse_parts() {
        let (parts, symbols) = Schematic::parse_line(1, "467..114..");
        assert_eq!(
            parts,
            vec![
                Part {
                    number: 467,
                    row: 1,
                    start: 0,
                    end: 3,
                },
                Part {
                    number: 114,
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
        let (parts, symbols) = Schematic::parse_line(3, "...$.*....");

        assert_eq!(parts.len(), 0);
        assert_eq!(
            symbols,
            vec![Symbol { row: 3, column: 3 }, Symbol { row: 3, column: 5 }]
        );
    }

    #[test]
    fn test_parse_mixed() {
        let (parts, symbols) = Schematic::parse_line(8, "617*......");
        assert_eq!(
            parts,
            vec![Part {
                number: 617,
                row: 8,
                start: 0,
                end: 3,
            }]
        );
        assert_eq!(symbols, vec![Symbol { row: 8, column: 3 }])
    }

    #[test]
    fn test_adjacent_top() {
        let part = Part {
            number: 617,
            row: 8,
            start: 0,
            end: 3,
        };

        for i in 0..3 {
            let symbol = Symbol { row: 7, column: i };
            assert!(part.is_adjacent(&symbol), "{} should be adjacent", i);
        }
    }

    #[test]
    fn test_adjacent_bot() {
        let part = Part {
            number: 617,
            row: 8,
            start: 0,
            end: 3,
        };

        for i in 0..3 {
            let symbol = Symbol { row: 9, column: i };
            assert!(part.is_adjacent(&symbol), "{} should be adjacent", i);
        }
    }
}
