use fib::fib_sequence;
use num::BigUint;

use crate::pull_request::PullRequest;
use crate::comment::post_comment;
// use octocrab::{
//     models::{pulls::PullRequest, repos::Content, repos::DiffEntry},
//     Octocrab, Page,
// };
use std::env;
use std::process;
//use crate::regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // if args.len() != 3 {
    //     eprintln!("Usage: <enable_fib> <max_threshhold>");
    //     process::exit(1);
    // }

    let pr_number = 2; //env::var("PR_NUMBER").expect("GITHUB_EVENT_NUMBER not set");
    //let pr_number = pr_number.parse::<u32>().expect("GITHUB_EVENT_NUMBER is not a valid number");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let owner = env::var("GITHUB_REPOSITORY_OWNER").expect("GITHUB_REPOSITORY_OWNER not set");
    let token= env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let token= token.as_str();

    // let enable_fib = match enable_fib.parse::<bool>() {
    //     Ok(value) => value,
    //     Err(_) => {
    //         eprintln!("Invalid value for enable_fib. Expected 'true' or 'false'.");
    //         process::exit(1);
    //     }
    // };

    // let max_threshhold = match max_threshhold.parse::<u32>() {
    //     Ok(value) => value,
    //     Err(_) => {
    //         eprintln!("Invalid value for max_threshhold. Expected a positive integer.");
    //         process::exit(1);
    //     }
    // };

    // println!("Verbose: {}", enable_fib);
    // println!("Limit: {}", max_threshhold);

    let sample_string = "This is a sample PR content with numbers 123, -456, and 789.";
    let numbers = numbers::extract_numbers_from_string(sample_string);
    println!("Extracted numbers: {:?}", numbers);

    let _n = 100;
    //println!("Fibonacci({}) = {}", n, fib::fib_sequence(n));
    //println!("Fibonacci sequence: {:?}", fib::fib_sequence(95));

    let pr = octocrab::instance()
        .pulls("JOELNATHAN544", "fibbot")
        .list_files(1)
        .await?;
    println!("{:?}", pr);
    let path = &pr.items.first().unwrap().patch.clone().unwrap();
    let number = numbers::extract_numbers_from_string(&path);
    println!("The numbers fron pull request are {:?}", number);
    // for i in 0..number.len() {
        // println!(
        //     "{}. FIbonnacci of {} is {}",
        //     1 + 1,
        //     number[1],
        //     fib::fib_sequence(number[1] as u64)
        // );
    // }
    //let fibonacci_results = numbers.iter().map(|&num| (num, fib_sequence(num as u64))).collect::<Vec<_>>();
    let fibonacci_results = numbers.iter().map(|&num| (1, 2)).collect::<Vec<_>>();
    //let fibonacci_results:Vec<i32, BigUint> = 2;
    let comment_body = fibonacci_results.iter()
        .fold(String::from("### Fibonacci Computations:\n"), |mut acc, (num, result)| {
            acc.push_str(&format!("- Fibonacci({}) = {}\n", num, result));
            acc
        });

    // let fibonacci_results = numbers.iter().map(|&num| (num, fibonacci_iterative(num))).collect::<Vec<_>>();

    // // let comment_body = fibonacci_results.iter()
    // //     .fold(String::from("### Fibonacci Computations:\n"), |mut acc, (num, result)| {
    // //         acc.push_str(&format!("- Fibonacci({}) = {}\n", num, result));
    // //         acc
    // //     });

    if let Err(e) = post_comment(pr_number, token, &comment_body).await {
        eprintln!("Error posting comment: {}", e);
    }
    Ok(())
}

fn compute_results() -> String {
    // Example computation: sum of two numbers
    let result = 42 + 58;
    format!("The  computed result is: {}", result)
}

mod comment;
mod fib;
mod numbers;
mod pull_request;
