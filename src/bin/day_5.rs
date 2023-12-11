use std::{fmt::Debug, ops::Range, str::FromStr};

fn main() {
    println!("Day 5 Star 1: {}", 0);
}

struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn get(&self, input: isize) -> isize {
        for range in &self.ranges {
            if let Some(output) = range.get(input) {
                return output;
            }
        }
        input
    }
}

struct MappingRange {
    range: Range<isize>,
    offset: isize,
}

impl MappingRange {
    fn parse(inp: &str) -> MappingRange {
        let mut parts = inp.split_whitespace();
        let dest_start = extract::<isize>(parts.next());
        let src_start = extract::<isize>(parts.next());
        let length = extract::<isize>(parts.next());

        MappingRange {
            range: src_start..src_start + length,
            offset: dest_start - src_start,
        }
    }

    fn get(&self, input: isize) -> Option<isize> {
        if self.range.contains(&input) {
            Some(input + self.offset)
        } else {
            None
        }
    }
}

fn extract<T>(inp: Option<&str>) -> T
where
    <T as FromStr>::Err: Debug,
    T: FromStr,
{
    inp.unwrap().parse::<T>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_within_range() {
        let mapping = Mapping {
            ranges: vec![
                MappingRange::parse("50 98 2"),
                MappingRange::parse("52 50 48"),
            ],
        };
        // First Range
        assert_eq!(mapping.get(98), 50);
        // Second Range
        assert_eq!(mapping.get(51), 53);
    }

    #[test]
    fn test_mapping_beyond_range() {
        let mapping = Mapping {
            ranges: vec![
                MappingRange::parse("50 98 2"),
                MappingRange::parse("52 50 48"),
            ],
        };
        assert_eq!(mapping.get(49), 49);
        assert_eq!(mapping.get(100), 100);
    }

    #[test]
    fn test_mapping_range_within_range() {
        let mapping = MappingRange::parse("50 98 2");
        assert_eq!(mapping.get(98), Some(50));
        assert_eq!(mapping.get(99), Some(51));
    }

    #[test]
    fn test_mapping_range_beyond_range() {
        let mapping = MappingRange::parse("50 98 2");
        assert_eq!(mapping.get(97), None);
        assert_eq!(mapping.get(100), None);
    }
}
