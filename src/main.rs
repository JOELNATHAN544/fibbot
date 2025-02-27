use std::env::{self, args};

/*use clap::{builder::Str, Parser};

#[derive(Parser, Debug)]
#[command(name = "my_rust_action")]
#[command(about = "A GitHub Action written in Rust", long_about = None)]
struct Cli {
    #[arg(long, default_value_t = false)]
    enable_fib: bool,

    #[arg(long, default_value_t = 100)]
    max_threshhold: u32,
}*/

fn main() {
    //let arg = Cli::parse();
    let args: Vec<String> = env::args().collect();
   
        //eprintln!("Please enter two arguments <enable_fib> <max_threshhold>");
   
        let enable_fib = &args[1];
        let max_threshhold = &args[2];

        // if args.len() != 3 {
        //     eprintln!("Please enter two arguments <enable_fib> <max_threshhold>");
        //     std::process::exit(1);
        // }
        println!("Verbose: {}", enable_fib);
        println!("Limit: {}", max_threshhold);
    
}
