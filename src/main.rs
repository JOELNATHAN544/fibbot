use crate::pull_request::PullRequest;
// use octocrab::{
//     models::{pulls::PullRequest, repos::Content, repos::DiffEntry},
//     Octocrab, Page,
// };
use std::env;
use std::process;
//use crate::regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error >>{

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
    let numbers = numbers::extract_numbers_from_string(sample_string);
    println!("Extracted numbers: {:?}", numbers);
    for i in 0..numbers.len() {
        // if i < 0 {
        //     println!(
        //         "{}. FIbonnacci of positive {} is {}",
        //         i + 1,
        //         numbers[i],
        //         fib::fib_sequence(-numbers[i] as u64)
        //     );
        // }
        println!(
            "{}. FIbonnacci of {} is {}",
            i + 1,
            numbers[i],
            fib::fib_sequence(numbers[i] as u64)
        );
    }
    let _n = 100;
    //println!("Fibonacci({}) = {}", n, fib::fib_sequence(n));
    //println!("Fibonacci sequence: {:?}", fib::fib_sequence(95));

    let pr = octocrab::instance()
        .pulls("lele-maxwell", "fibbot-test")
        .list_files(1)
        .await?;
    println!("{:?}", pr);
    let path = &pr.items.first().unwrap().patch.clone().unwrap();
    let numbers = numbers::extract_numbers_from_string(&path);
    println!("{:?}", numbers);
    Ok(())
}

fn compute_results() -> String {
    // Example computation: sum of two numbers
    let result = 42 + 58;
    format!("The computed result is: {}", result)
}

mod comment;
mod fib;
mod numbers;
mod pull_request;
