use std::collections::HashSet;
use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct Problem {
    id: String,
    contest_id: String,
}

fn standard_contest_number(contest_id: &str, prefix: &str) -> Option<u32> {
    contest_id
        .strip_prefix(prefix)
        .and_then(|number| number.parse().ok())
}

#[test]
fn abc458_and_abc459_exist_in_problem_data_and_models() {
    let problems_text = fs::read_to_string("data/problems.json").unwrap();
    let problems: Vec<Problem> = serde_json::from_str(&problems_text).unwrap();

    let problem_models_text = fs::read_to_string("data/problem-models.json").unwrap();
    let problem_models: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&problem_models_text).unwrap();

    let mut seen = HashSet::new();
    let mut duplicated = Vec::new();
    for problem in &problems {
        if !seen.insert(problem.id.as_str()) {
            duplicated.push(problem.id.as_str());
        }
    }
    assert!(
        duplicated.is_empty(),
        "duplicated problem ids: {duplicated:?}"
    );

    for problem in &problems {
        assert!(
            problem_models.contains_key(&problem.id),
            "{} is missing from problem-models.json",
            problem.id
        );
    }

    for contest in ["abc458", "abc459"] {
        for task in ["a", "b", "c", "d", "e", "f", "g"] {
            let id = format!("{contest}_{task}");
            assert!(
                problems
                    .iter()
                    .any(|problem| problem.id == id && problem.contest_id == contest),
                "{id} is missing from problems.json"
            );
            assert!(
                problem_models.contains_key(&id),
                "{id} is missing from problem-models.json"
            );
        }
    }
}

#[test]
fn problem_data_is_not_older_than_known_current_standard_contests() {
    let problems_text = fs::read_to_string("data/problems.json").unwrap();
    let problems: Vec<Problem> = serde_json::from_str(&problems_text).unwrap();

    for (prefix, expected_latest) in [("abc", 465), ("arc", 223), ("agc", 77)] {
        let latest = problems
            .iter()
            .filter_map(|problem| standard_contest_number(&problem.contest_id, prefix))
            .max()
            .unwrap_or(0);

        assert!(
            latest >= expected_latest,
            "{prefix} data is stale: latest {latest}, expected at least {expected_latest}"
        );
    }
}
