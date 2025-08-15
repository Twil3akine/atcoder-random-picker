use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, path::PathBuf, println, string::String, vec::Vec};

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
    // カレントディレクトリ基準で src/data を指す
    let mut problems_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    problems_path.push("src/data/problems.json");

    let mut problem_models_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    problem_models_path.push("src/data/problem-models.json");

    // ファイル読み込み
    let problems_text = fs::read_to_string(problems_path)?;
    let problems: Vec<Problem> = serde_json::from_str(&problems_text)?;

    let problem_models_text = fs::read_to_string(problem_models_path)?;
    let problem_models: HashMap<String, ProblemModel> = serde_json::from_str(&problem_models_text)?;

    // 確認出力
    for (id, model) in &problem_models {
        println!("{}: {:?}", id, model.difficulty);
    }

    Ok((problems, problem_models))
}
