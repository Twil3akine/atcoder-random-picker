use serde::Deserialize;
use std::{error::Error, marker::Sync};

#[derive(Debug, Deserialize)]
pub struct Problem {
    pub id: String,
    pub contest_id: String,
    pub title: String,
}

pub async fn fetch_problem() -> Result<Vec<Problem>, Box<dyn Error + Send + Sync>> {
    let url = "https://kenkoooo.com/atcoder/resources/problems.json";

    // reqwest::Clientを作成し、User-Agentヘッダーを追加
    let client = reqwest::Client::builder()
        .user_agent("atcoder-random-picker/0.1") // 任意の名前やバージョンを指定
        .build()?;

    let text = client.get(url).send().await?.text().await?;
    let problem: Vec<Problem> = serde_json::from_str(&text)?;

    Ok(problem)
}