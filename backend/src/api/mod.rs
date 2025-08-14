use serde::Deserialize;
use std::{error::Error, marker::Sync};

#[derive(Debug, Deserialize)]
pub struct Problem {
	pub id: String,
	pub contest_id: String,
	pub title: String,
}

pub async fn fetch_problem() -> Result<Vec<Problem>, Box<dyn Error+Send+Sync>> {
	let url = "https://kenkoooo.com/atcoder/resources/problems.json";
	let text = reqwest::get(url).await?.text().await?;
	let problem: Vec<Problem> = serde_json::from_str(&text)?;

	Ok(problem)
}