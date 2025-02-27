use std::env;
use std::process;
//use crate::regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: <enable_fib> <max_threshhold>");
        process::exit(1);
    }

    let enable_fib = &args[1];
    let max_threshhold = &args[2];

    let enable_fib = match enable_fib.parse::<bool>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid value for enable_fib. Expected 'true' or 'false'.");
            process::exit(1);
        }
    };

    let max_threshhold = match max_threshhold.parse::<u32>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid value for max_threshhold. Expected a positive integer.");
            process::exit(1);
        }
    };

    println!("Verbose: {}", enable_fib);
    println!("Limit: {}", max_threshhold);

    let sample_string = "This is a sample PR content with numbers 123, -456, and 789.";
    let numbers = regex::extract_numbers_from_string(sample_string);
    println!("Extracted numbers: {:?}", numbers);
}

mod regex;
