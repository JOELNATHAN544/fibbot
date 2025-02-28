// use regex::Regex;

// pub fn extract_numbers_from_string(input: &str) -> Vec<i32> {
//     // Define a regular expression to match integers
//     let re = Regex::new(r"-?\d+").unwrap();

//     // Find all matches and convert them to integers
//     re.find_iter(input)
//         .filter_map(|mat| mat.as_str().parse::<i32>().ok())
//         .collect()
// }

// // fn extract_numerical_values(pr: &PullRequest) -> (u32, u32, usize) {
// //     let total_additions = pr.files.iter().map(|file| file.additions).sum();
// // }

pub fn extract_numbers_from_string(input: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.  is_empty() {
            if let Ok(num) = current_number.parse::<i32>() {
                numbers.push(num);
            }
            current_number.clear();
        }
    }

    // Check if there's a number left at the end of the string
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<i32>() {
            numbers.push(num);
        }
    }

    numbers
}