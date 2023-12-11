use std::{fmt::Debug, ops::Range};

use once_cell::sync::Lazy;
use regex::Regex;

use advent_of_code_2023::{io::read_lines, regex};

fn main() {
    let input = read_lines("inputs/day_5.txt");
    let almanac = Almanac::parse(input);

    let star_1 = almanac
        .seeds
        .iter()
        .map(|seed_id| almanac.seed(*seed_id).location)
        .min();

    println!("Day 5 Star 1: {}", star_1.unwrap());
}

struct Almanac {
    seeds: Vec<isize>,
    seed_to_soil: Mapping,
    soil_to_fertilizer: Mapping,
    fertilizer_to_water: Mapping,
    water_to_light: Mapping,
    light_to_temperature: Mapping,
    temperature_to_humidity: Mapping,
    humidity_to_location: Mapping,
}

impl Default for Almanac {
    fn default() -> Self {
        Almanac {
            seeds: vec![],
            seed_to_soil: Mapping { ranges: vec![] },
            soil_to_fertilizer: Mapping { ranges: vec![] },
            fertilizer_to_water: Mapping { ranges: vec![] },
            water_to_light: Mapping { ranges: vec![] },
            light_to_temperature: Mapping { ranges: vec![] },
            temperature_to_humidity: Mapping { ranges: vec![] },
            humidity_to_location: Mapping { ranges: vec![] },
        }
    }
}

impl Almanac {
    fn parse<T: Iterator<Item = U>, U: AsRef<str>>(mut lines: T) -> Almanac {
        let mut result = Almanac::default();

        loop {
            let next_line = lines.next();
            if next_line.is_none() {
                break;
            }

            let line = next_line.unwrap();
            let line = line.as_ref().trim();

            if line.is_empty() {
                continue;
            }

            if let Some(seeds) = parse_seeds(line) {
                result.seeds = seeds;
            } else if line.starts_with("seed-to-soil map") {
                result.seed_to_soil = parse_maps(&mut lines);
            } else if line.starts_with("soil-to-fertilizer map") {
                result.soil_to_fertilizer = parse_maps(&mut lines);
            } else if line.starts_with("fertilizer-to-water map") {
                result.fertilizer_to_water = parse_maps(&mut lines);
            } else if line.starts_with("water-to-light map") {
                result.water_to_light = parse_maps(&mut lines);
            } else if line.starts_with("light-to-temperature map") {
                result.light_to_temperature = parse_maps(&mut lines);
            } else if line.starts_with("temperature-to-humidity map") {
                result.temperature_to_humidity = parse_maps(&mut lines);
            } else if line.starts_with("humidity-to-location map") {
                result.humidity_to_location = parse_maps(&mut lines);
            }
        }

        result
    }

    fn seed(&self, id: isize) -> Seed {
        let soil = self.seed_to_soil.get(id);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        let location = self.humidity_to_location.get(humidity);

        Seed {
            id,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location,
        }
    }
}

static SEED_REGEX: Lazy<Regex> = regex!(r"seeds: (?<seeds>.+)");

fn parse_seeds(line: &str) -> Option<Vec<isize>> {
    SEED_REGEX.captures(line).map(|captures| {
        captures
            .name("seeds")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>()
    })
}

fn parse_maps<T: Iterator<Item = U>, U: AsRef<str>>(lines: &mut T) -> Mapping {
    let ranges = lines
        .map_while(|line| MappingRange::try_parse(line.as_ref()))
        .collect::<Vec<_>>();
    Mapping { ranges }
}

#[derive(Debug, PartialEq)]
struct Seed {
    id: isize,
    soil: isize,
    fertilizer: isize,
    water: isize,
    light: isize,
    temperature: isize,
    humidity: isize,
    location: isize,
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
    fn try_parse(inp: &str) -> Option<MappingRange> {
        let mut parts = inp.split_whitespace();
        let dest_start = parts.next()?.parse::<isize>().ok()?;
        let src_start = parts.next()?.parse::<isize>().ok()?;
        let length = parts.next()?.parse::<isize>().ok()?;

        Some(MappingRange {
            range: src_start..src_start + length,
            offset: dest_start - src_start,
        })
    }

    fn get(&self, input: isize) -> Option<isize> {
        if self.range.contains(&input) {
            Some(input + self.offset)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almanac_parse_seeds() {
        let almanac = Almanac::parse("seeds: 79 14 55 13".lines());
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
    }

    const TEST_INPUT: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn test_almanac_getting_seed() {
        let almanac = Almanac::parse(TEST_INPUT.lines());

        assert_eq!(
            almanac.seed(79),
            Seed {
                id: 79,
                soil: 81,
                fertilizer: 81,
                water: 81,
                light: 74,
                temperature: 78,
                humidity: 78,
                location: 82,
            }
        );
    }

    #[test]
    fn test_mapping_within_range() {
        let mapping = Mapping {
            ranges: vec![
                MappingRange::try_parse("50 98 2").unwrap(),
                MappingRange::try_parse("52 50 48").unwrap(),
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
                MappingRange::try_parse("50 98 2").unwrap(),
                MappingRange::try_parse("52 50 48").unwrap(),
            ],
        };
        assert_eq!(mapping.get(49), 49);
        assert_eq!(mapping.get(100), 100);
    }

    #[test]
    fn test_mapping_range_within_range() {
        let mapping = MappingRange::try_parse("50 98 2").unwrap();
        assert_eq!(mapping.get(98), Some(50));
        assert_eq!(mapping.get(99), Some(51));
    }

    #[test]
    fn test_mapping_range_beyond_range() {
        let mapping = MappingRange::try_parse("50 98 2").unwrap();
        assert_eq!(mapping.get(97), None);
        assert_eq!(mapping.get(100), None);
    }
}
