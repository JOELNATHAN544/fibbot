// use reqwest::blocking::Client;
// use std::env;
// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// pub struct PullRequestFile {
//     pub filename: String,
//     pub status: String,
//     pub additions: u32,
//     pub deletions: u32,
//     pub changes: u32,
//     pub blob_url: String,
//     pub raw_url: String,
//     pub contents_url: String,
// }

// #[derive(Debug)]
// pub struct PullRequest {
//     pub files: Vec<PullRequestFile>,
// }

// // src/lib.rs

// pub fn fetch_pr_content(repo: &str, pr_number: u32, token: &str) -> Result<PullRequest, reqwest::Error> {
//     let url = format!("https://api.github.com/repos/{}/pulls/{}/files", repo, pr_number);
//     let client = Client::new();
//     let response = client.get(&url)
//         .header("Accept", "application/vnd.github+json")
//         .header("Authorization", format!("Bearer {}", token))
//         .header("X-GitHub-Api-Version", "2022-11-28")
//         .send()?;

//     if response.status().is_success() {
//         let files: Vec<PullRequestFile> = response.json()?;
//         Ok(PullRequest { files })
//     } else {
//         Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
//     }
// }

// // src/lib.rs
// pub fn extract_numerical_values(pr: &PullRequest) -> (u32, u32, usize) {
//     let total_additions = pr.files.iter().map(|file| file.additions).sum();
//     let total_deletions = pr.files.iter().map(|file| file.deletions).sum();
//     let total_changes = pr.files.len();

//     (total_additions, total_deletions, total_changes)
// }