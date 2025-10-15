use std::option::Option::None;
use std::path::PathBuf;
use std::string::ToString;
use std::{env, fs};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Problem {
    pub id: String,
    pub contest_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProblemModelRaw {
    pub difficulty: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ProblemModel {
    pub difficulty: Option<f64>,
}

fn adjust_difficulty(difficulty: Option<i32>) -> Option<f64> {
    match difficulty {
        Some(d) if d >= 400 => Some(d as f64),
        Some(d) => Some(400.0 / f64::exp(1.0 - d as f64 / 400.0)),
        None => None,
    }
}

pub async fn fetch_problem() -> Result<(Vec<Problem>, HashMap<String, ProblemModel>), Box<dyn Error + Send + Sync>> {
    let data_dir = env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
    
    let base_path = PathBuf::from(data_dir);

    let problems_path = base_path.join("problems.json");
    let problem_models_path = base_path.join("problem-models.json");

    // ファイルが存在しない場合に落ちないようにする
    if !problems_path.exists() {
        eprintln!("Warning: {} not found", problems_path.display());
        return Ok((Vec::new(), HashMap::new()));
    }
    if !problem_models_path.exists() {
        eprintln!("Warning: {} not found", problem_models_path.display());
        return Ok((Vec::new(), HashMap::new()));
    }

    // ファイル読み込み
    let problems_text = fs::read_to_string(problems_path)?;
    let problems: Vec<Problem> = serde_json::from_str(&problems_text)?;

    let problem_models_text = fs::read_to_string(problem_models_path)?;
    let raw_models: HashMap<String, ProblemModelRaw> = serde_json::from_str(&problem_models_text)?;

    let problem_models: HashMap<String, ProblemModel> = raw_models
        .into_iter()
        .map(|(id, raw)| {
            (
                id,
                ProblemModel {
                    difficulty: adjust_difficulty(raw.difficulty),
                },
            )
        })
        .collect();

    Ok((problems, problem_models))
}