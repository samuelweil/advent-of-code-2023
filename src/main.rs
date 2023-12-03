mod day1;

use std::{
    env,
    fs::File,
    io::{self, BufRead},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a day number");
        exit(1);
    }

    let day = args[1]
        .parse::<u32>()
        .expect("Please provide a valid day number");
    println!("Running day {}", day);

    let file_name = format!("inputs/day_{}.txt", day);
    let file = File::open(&file_name).expect(format!("Could not open {}", &file_name).as_str());
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());

    match day {
        1 => day1::run(lines),
        _ => println!("Day {} not implemented", day),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        assert!(true);
    }
}
