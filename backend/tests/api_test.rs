use backend::api::{Problem, ProblemModel};
use std::collections::HashMap;

/// fetch_problem のモック版
pub async fn fetch_problem_mock() -> (Vec<Problem>, HashMap<String, ProblemModel>) {
    // ダミーの Problem
    let problems = vec![
        Problem {
            id: "abc_001".to_string(),
            contest_id: "contest1".to_string(),
            name: "Test Problem 1".to_string(),
        },
        Problem {
            id: "def_002".to_string(),
            contest_id: "contest2".to_string(),
            name: "Test Problem 2".to_string(),
        },
    ];

    // ダミーの ProblemModel
    let mut problem_models = HashMap::new();
    problem_models.insert(
        "abc_001".to_string(),
        ProblemModel { difficulty: Some(400.0) },
    );
    problem_models.insert(
        "def_002".to_string(),
        ProblemModel { difficulty: Some(800.0) },
    );

    (problems, problem_models)
}

#[tokio::test]
async fn test_fetch_problem_mock() {
    let (problems, problem_models) = fetch_problem_mock().await;

    assert!(!problems.is_empty());
    assert!(!problem_models.is_empty());

    let first = &problems[0];
    assert_eq!(first.id, "abc_001");
    assert_eq!(first.contest_id, "contest1");
    assert_eq!(first.name, "Test Problem 1");

    let first_model = problem_models.get(&first.id).unwrap();
    assert_eq!(first_model.difficulty, Some(400.0));
}