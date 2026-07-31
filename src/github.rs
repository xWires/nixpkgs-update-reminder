use std::error::Error;
use chrono::{DateTime, Utc};
use serde_json::Value;

pub async fn get_commit_date(owner: &str, repo: &str, commit_hash: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let response = reqwest::Client::new()
        .get(format!("https://api.github.com/repos/{owner}/{repo}/commits/{commit_hash}"))
        .header("User-Agent", "nixpkgs-update-reminder")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()
        .await?;

    let json: Value = serde_json::from_str(&response.text().await?)?;

    let raw_commit_date = json["commit"]["committer"]["date"].as_str().ok_or("failed to get commit date from GitHub API response")?;

    Ok(DateTime::parse_from_str(raw_commit_date, "%+")?.to_utc())
}
