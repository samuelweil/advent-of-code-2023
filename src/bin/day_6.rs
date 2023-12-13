fn main() {
    let races = vec![
        Race {
            time: 41,
            max_distance: 249,
        },
        Race {
            time: 77,
            max_distance: 1362,
        },
        Race {
            time: 70,
            max_distance: 1127,
        },
        Race {
            time: 96,
            max_distance: 1011,
        },
    ];

    let star_1: usize = races
        .iter()
        .map(|race| race.ways_to_beat_record())
        .product();
    println!("Day 6 - Star 1: {}", star_1);
}

struct Race {
    time: usize,
    max_distance: usize,
}

impl Race {
    /// Returns the distances covered by holding the boat for a given time.
    ///
    /// The index in the array is the time held while the value is the distance
    fn distances(&self) -> Vec<usize> {
        (0..(self.time as usize + 1))
            .map(|time_held| {
                let speed = time_held;
                let time_remaining = self.time - time_held;
                speed * time_remaining
            })
            .collect()
    }

    fn ways_to_beat_record(&self) -> usize {
        self.distances()
            .into_iter()
            .filter(|distance| *distance > self.max_distance)
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::Race;

    #[test]
    fn test_race_distances() {
        let race = Race {
            time: 7,
            max_distance: 9,
        };

        assert_eq!(race.distances(), vec![0, 6, 10, 12, 12, 10, 6, 0]);
    }

    #[test]
    fn test_race_ways_to_beat_record() {
        let race = Race {
            time: 7,
            max_distance: 9,
        };

        assert_eq!(race.ways_to_beat_record(), 4);
    }
}
