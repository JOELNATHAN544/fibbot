// use crate::comment::Comment;
// use reqwest::Error;
// use serde_json::json;

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

pub struct PullRequest;
use octocrab::{models::repos::DiffEntry, Page};
impl PullRequest {
    pub async fn get_pr(owner: &str, repo: &str) -> Result<Page<DiffEntry>, octocrab::Error> {
      octocrab::instance().pulls("JOELNATHAN544", "fibbot").list_files(1).await
    }
}