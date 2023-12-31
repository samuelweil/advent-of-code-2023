#[macro_export]
macro_rules! lazy {
    ( $x: expr) => {
        Lazy::new(|| $x)
    };
}

#[macro_export]
macro_rules! regex {
    ( $x:literal) => {
        Lazy::new(|| Regex::new($x).unwrap())
    };
}

pub mod io {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn read_lines(filename: &str) -> impl Iterator<Item = String> {
        let file = File::open(filename).expect(format!("Could not open {}", &filename).as_str());
        BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not parse line"))
    }
}
