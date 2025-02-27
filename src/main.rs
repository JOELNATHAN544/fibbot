use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: <enable_fib> <max_threshhold>");
        process::exit(1);
    }

    let enable_fib = &args[1];
    let max_threshhold = &args[2];

    // Validate enable_fib (should be "true" or "false")
    let enable_fib = match enable_fib.parse::<bool>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid value for enable_fib. Expected 'true' or 'false'.");
            process::exit(1);
        }
    };

    // Validate max_threshhold (should be a positive integer)
    let max_threshhold = match max_threshhold.parse::<u32>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid value for max_threshhold. Expected a positive integer.");
            process::exit(1);
        }
    };

    // Log the validated parameters
    println!("Verbose: {}", enable_fib);
    println!("Limit: {}", max_threshhold);

    // Example: Perform some action based on the parameters
    // if enable_fib {
    //     println!("Fibonacci sequence enabled up to {}", max_threshhold);
    // } else {
    //     println!("Fibonacci sequence disabled");
    // }
}