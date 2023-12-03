mod day1;

use std::{env, process::exit};

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

    match day {
        1 => day1::run(String::from("")),
        _ => eprintln!("Day {} not implemented", day),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        assert!(true);
    }
}
