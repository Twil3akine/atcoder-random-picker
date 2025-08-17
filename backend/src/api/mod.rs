use serde::Deserialize;
use core::prelude::v1::derive;
use std::{collections::HashMap, error::Error, fs, iter::{IntoIterator, Iterator}, option::Option::{self, None, Some}, path::PathBuf, string::String, vec::Vec};

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
    // カレントディレクトリ基準で src/data を指す
    let mut problems_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    problems_path.push("../data/problems.json");

    let mut problem_models_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    problem_models_path.push("../data/problem-models.json");

    // ファイル読み込み
    let problems_text = fs::read_to_string(problems_path)?;
    let problems: Vec<Problem> = serde_json::from_str(&problems_text)?;

    let problem_models_text = fs::read_to_string(problem_models_path)?;
    let raw_models: HashMap<String, ProblemModelRaw> = serde_json::from_str(&problem_models_text)?;

    // 補正式を適用させる
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
