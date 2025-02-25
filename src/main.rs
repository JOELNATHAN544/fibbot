use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "my_rust_action")]
#[command(about = "A GitHub Action written in Rust", long_about = None)]
struct Cli {
    #[arg(long, default_value_t = false)]
    enable_fib: bool,

    #[arg(long, default_value_t = 100)]
    max_threshhold: u32,
}

fn main() {
    let args = Cli::parse();
    println!("Verbose: {}", args.enable_fib);
    println!("Limit: {}", args.max_threshhold);
}

