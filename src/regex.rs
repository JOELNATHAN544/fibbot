use regex::Regex;

pub fn extract_numbers_from_string(input: &str) -> Vec<i32> {
    // Define a regular expression to match integers
    let re = Regex::new(r"-?\d+").unwrap();

    // Find all matches and convert them to integers
    re.find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect()
} 