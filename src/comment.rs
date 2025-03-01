// use reqwest::Error;
// use serde_json::json;
// use serde::Serialize;

// #[derive(Serialize)]
// pub struct Comment {
//     pub body: String,
// }

// //use crate::comment::Comment;

// pub fn post_comment(
//     owner: &str,
//     repo: &str,
//     pull_number: u64,
//     body: &str,
//     token: &str,
// ) -> Result<(), Error> {
//     let url = format!(
//         "https://api.github.com/repos/{}/{}/issues/{}/comments",
//         owner, repo, pull_number
//     );

//     let client = reqwest::blocking::Client::new();
//     let comment = Comment {
//         body: body.to_string(),
//     };

//     let response = client
//         .post(&url)
//         .header("Authorization", format!("token {}", token))
//         .header("User-Agent", "my-awesome-agent/1.0")
//         .json(&comment)
//         .send()?;

//     if response.status().is_success() {
//         println!("Comment posted successfully");
//     } else {
//         eprintln!("Failed to post comment: {}", response.status());
//     }

//     Ok(())
// }

use reqwest::Client;
use std::{env, error::Error};

pub async fn post_comment(pr_number: u32, token: &str, comment_body: &str) -> Result<(), Box<dyn Error>> {
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let owner = env::var("GITHUB_REPOSITORY_OWNER").expect("GITHUB_REPOSITORY_OWNER not set");
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        owner, repo, std::env::var("GITHUB_PR_NUMBER")?,
      
      
    );

    // Debug: Print URL and token
    println!("Posting comment to URL: {} ", url);
    println!("Using token: {}", token);

    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "fibbot")
        .json(&serde_json::json!({
            "body": comment_body
        }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Comment posted successfully.");
    } else {
        eprintln!("Failed to post comment: {}", response.text().await?);
    }

    Ok(())
    
}

