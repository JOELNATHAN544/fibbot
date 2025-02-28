use reqwest::Error;
use serde_json::json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Comment {
    pub body: String,
}

//use crate::comment::Comment;

pub fn post_comment(
    owner: &str,
    repo: &str,
    pull_number: u64,
    body: &str,
    token: &str,
) -> Result<(), Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        owner, repo, pull_number
    );

    let client = reqwest::blocking::Client::new();
    let comment = Comment {
        body: body.to_string(),
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "my-awesome-agent/1.0")
        .json(&comment)
        .send()?;

    if response.status().is_success() {
        println!("Comment posted successfully");
    } else {
        eprintln!("Failed to post comment: {}", response.status());
    }

    Ok(())
}
