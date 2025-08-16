#![allow(unused)]

use backend::api::{fetch_problem, Problem, ProblemModel};
use std::collections::HashMap;

#[tokio::test]
async fn test_fetch_problem_returns_data() {
    let result = fetch_problem().await;

    assert!(result.is_ok());
    let (problems, problem_models): (Vec<Problem>, HashMap<String, ProblemModel>) = result.unwrap();

    // 空でないことを確認
    assert!(!problems.is_empty());
    assert!(!problem_models.is_empty());

    // Problem の中身確認
    let first_problem = &problems[0];
    assert!(!first_problem.id.is_empty());
    assert!(!first_problem.contest_id.is_empty());
    assert!(!first_problem.name.is_empty());
}
