use serde::Deserialize;
use std::{collections::HashMap, error::Error, println, string::String, vec::Vec};

#[derive(Debug, Deserialize)]
pub struct Problem {
    pub id: String,
    pub contest_id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ProblemModel {
	pub difficulty: Option<i32>,
}

pub async fn fetch_problem() -> Result<(Vec<Problem>, HashMap<String, ProblemModel>), Box<dyn Error + Send + Sync>> {
    // let problems_url = "https://kenkoooo.com/atcoder/resources/problems.json";
	// let problem_models_url = "https://kenkoooo.com/atcoder/resources/problem-models.json";
	let problems_url = "https://github.com/Twil3akine/atcoder-random-picker/blob/master/.gitconfig";
	let problem_models_url = "https://github.com/Twil3akine/atcoder-random-picker/blob/master/.gitconfig";

    let client = reqwest::Client::builder()
		.user_agent("atcoder-random-picker/0.1 (twil3; contact: twil3akine@gmail.com)")
	    .build()?;

	// 問題一覧
    let problems_text = client.get(problems_url).send().await?.text().await?;
    let problems: Vec<Problem> = serde_json::from_str(&problems_text)?;

	// 問題モデル
	let problem_models_text = client.get(problem_models_url).send().await?.text().await?;
	let problem_models: HashMap<String, ProblemModel> = serde_json::from_str(&problem_models_text)?;

	for (id, model) in &problem_models {
		println!("{}: {:?}", id, model.difficulty);
	}

    Ok((problems, problem_models))
}