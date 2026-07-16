#![allow(unused)]

use backend::utils::api::{Problem, ProblemModel};
use backend::utils::routing::{router, AppState};
use hyper::{Body, Method, Request, StatusCode};
use std::assert;
use std::collections::HashMap;
use std::sync::Arc;

fn build_test_state() -> Arc<AppState> {
    let mut problem_models = HashMap::new();
    problem_models.insert(
        "abc001_a".to_string(),
        ProblemModel {
            difficulty: Some(1000.0),
        },
    );
    problem_models.insert(
        "arc001_a".to_string(),
        ProblemModel {
            difficulty: Some(1100.0),
        },
    );
    problem_models.insert(
        "agc001_a".to_string(),
        ProblemModel {
            difficulty: Some(1200.0),
        },
    );
    problem_models.insert(
        "abc212_a".to_string(),
        ProblemModel {
            difficulty: Some(900.0),
        },
    );
    problem_models.insert(
        "abc212_b".to_string(),
        ProblemModel {
            difficulty: Some(925.0),
        },
    );
    problem_models.insert(
        "abc213_a".to_string(),
        ProblemModel {
            difficulty: Some(850.0),
        },
    );
    problem_models.insert(
        "arc212_a".to_string(),
        ProblemModel {
            difficulty: Some(950.0),
        },
    );
    problem_models.insert(
        "typical90_a".to_string(),
        ProblemModel {
            difficulty: Some(700.0),
        },
    );
    problem_models.insert("abc459_a".to_string(), ProblemModel { difficulty: None });

    let problems = vec![
        Problem {
            id: "abc001_a".to_string(),
            contest_id: "abc001".to_string(),
            name: "A - Test Problem".to_string(),
        },
        Problem {
            id: "arc001_a".to_string(),
            contest_id: "arc001".to_string(),
            name: "A - ARC Test Problem".to_string(),
        },
        Problem {
            id: "agc001_a".to_string(),
            contest_id: "agc001".to_string(),
            name: "A - AGC Test Problem".to_string(),
        },
        Problem {
            id: "abc212_a".to_string(),
            contest_id: "abc212".to_string(),
            name: "A - Alloy".to_string(),
        },
        Problem {
            id: "abc212_b".to_string(),
            contest_id: "adt_all_20260615_2".to_string(),
            name: "B - Weak Password".to_string(),
        },
        Problem {
            id: "abc213_a".to_string(),
            contest_id: "abc213".to_string(),
            name: "A - Bitwise Exclusive Or".to_string(),
        },
        Problem {
            id: "arc212_a".to_string(),
            contest_id: "arc212".to_string(),
            name: "A - ARC Test Problem".to_string(),
        },
        Problem {
            id: "typical90_a".to_string(),
            contest_id: "typical90".to_string(),
            name: "A - Other Contest".to_string(),
        },
        Problem {
            id: "abc459_a".to_string(),
            contest_id: "abc459".to_string(),
            name: "A - Hell, World!".to_string(),
        },
    ];

    Arc::new(AppState {
        problems,
        problem_models,
    })
}

async fn build_and_send(method: Method, path: &str) -> (StatusCode, String) {
    let req = Request::builder()
        .method(method)
        .uri(path)
        .body(Body::empty())
        .unwrap();

    let state = build_test_state();
    let res = router(req, state).await.unwrap();

    let status = res.status();
    let body_bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    (status, body_string)
}

#[tokio::test]
async fn test_not_found_path() {
    let (status, body) = build_and_send(Method::GET, "/test").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "404 Not Found");
}

#[tokio::test]
async fn test_not_found_problem() {
    let (status, body) = build_and_send(Method::GET, "/?min=0&max=500").await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    #[derive(serde::Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let err: ErrorResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(err.message, "指定Diff範囲に該当する問題がありませんでした");
}

#[tokio::test]
async fn test_excluded_problem_is_not_selected() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=800&max=900&contest=abc&contest_from=212&contest_to=213&exclude=abc212_a",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(problem.id, "abc213_a");
}

#[tokio::test]
async fn test_all_matching_problems_excluded_returns_specific_message() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=800&max=900&contest=abc&contest_from=212&contest_to=213&exclude=abc212_a,abc213_a",
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    #[derive(serde::Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let error: ErrorResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(
        error.message,
        "履歴内の問題を除外すると、条件に一致する問題がありません。除外をOFFにするか、履歴を削除してください"
    );
}

#[tokio::test]
async fn test_empty_exclude_is_ignored() {
    let (status, _) = build_and_send(Method::GET, "/?min=900&max=900&exclude=").await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_duplicate_excluded_problem_ids_count_once() {
    let excluded = std::iter::repeat("abc212_a")
        .take(21)
        .collect::<Vec<_>>()
        .join(",");
    let path = format!("/?min=850&max=850&exclude={excluded}");
    let (status, body) = build_and_send(Method::GET, &path).await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(problem.id, "abc213_a");
}

#[tokio::test]
async fn test_more_than_twenty_excluded_problem_ids_is_bad_request() {
    let excluded = (0..21)
        .map(|index| format!("abc{index:03}_a"))
        .collect::<Vec<_>>()
        .join(",");
    let path = format!("/?exclude={excluded}");
    let (status, body) = build_and_send(Method::GET, &path).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'exclude' cannot contain more than 20 problem IDs.");
}

#[tokio::test]
async fn test_invalid_excluded_problem_id_is_bad_request() {
    let (status, body) = build_and_send(Method::GET, "/?exclude=abc212%2Fa").await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'exclude' contains an invalid problem ID.");
}

#[tokio::test]
async fn test_min_greater_than_max() {
    let (status, body) = build_and_send(Method::GET, "/?min=1500&max=500").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'min' cannot be greater than 'max'.");
}

#[tokio::test]
async fn test_negative_min_is_bad_request() {
    let (status, body) = build_and_send(Method::GET, "/?min=-1&max=500").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'min' cannot be less than 0.");
}

#[tokio::test]
async fn test_max_over_previous_upper_limit_is_allowed() {
    let (status, body) = build_and_send(Method::GET, "/?min=0&max=3855").await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        difficulty: f64,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert!(problem.difficulty <= 3855.0);
}

#[tokio::test]
async fn test_invalid_min_is_bad_request() {
    let (status, body) = build_and_send(Method::GET, "/?min=abc&max=500").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'min' must be a number.");
}

#[tokio::test]
async fn test_invalid_contest_from_is_bad_request() {
    let (status, body) = build_and_send(Method::GET, "/?contest_from=abc").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'contest_from' must be a positive integer.");
}

#[tokio::test]
async fn test_random_range() {
    let (status, body) = build_and_send(Method::GET, "/?min=500&max=1500").await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
        name: String,
        difficulty: f64,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();
    let diff = problem.difficulty;

    assert!(500.0 <= diff && diff <= 1500.0);
}

#[tokio::test]
async fn test_contest_number_from() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=900&max=900&contest=abc&contest_from=212&contest_to=212",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert_eq!(problem.id, "abc212_a");
    assert_eq!(problem.contest_id, "abc212");
}

#[tokio::test]
async fn test_reused_problem_uses_its_canonical_standard_contest() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=925&max=925&contest=abc&contest_from=212&contest_to=212",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert_eq!(problem.id, "abc212_b");
    assert_eq!(problem.contest_id, "abc212");
}

#[tokio::test]
async fn test_contest_number_to_without_from() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=0&max=1500&contest=abc,arc,agc&contest_to=1",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert!(["abc001_a", "arc001_a", "agc001_a"].contains(&problem.id.as_str()));
    assert!(["abc001", "arc001", "agc001"].contains(&problem.contest_id.as_str()));
}

#[tokio::test]
async fn test_empty_contest_from_is_ignored() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=0&max=1500&contest=abc,arc,agc&contest_from=&contest_to=1",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        contest_id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert!(["abc001", "arc001", "agc001"].contains(&problem.contest_id.as_str()));
}

#[tokio::test]
async fn test_agc_only_can_be_picked() {
    let (status, body) = build_and_send(Method::GET, "/?min=0&max=1500&contest=agc").await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        contest_id: String,
        difficulty: f64,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert!(problem.contest_id.starts_with("agc"));
    assert!(problem.difficulty <= 1500.0);
}

#[tokio::test]
async fn test_agc_only_with_large_max_is_not_bad_request() {
    let (status, body) = build_and_send(Method::GET, "/?min=0&max=999999&contest=agc").await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        contest_id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert!(problem.contest_id.starts_with("agc"));
}

#[tokio::test]
async fn test_agc_only_without_matching_difficulty_is_not_found() {
    let (status, body) = build_and_send(Method::GET, "/?min=1300&max=1400&contest=agc").await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    #[derive(serde::Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let err: ErrorResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(err.message, "指定Diff範囲に該当する問題がありませんでした");
}

#[tokio::test]
async fn test_multiple_contests_and_round_range_are_combined() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=800&max=1000&contest=abc,arc&contest_from=212&contest_to=212",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
        difficulty: f64,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert!(["abc212_a", "abc212_b", "arc212_a"].contains(&problem.id.as_str()));
    assert!(["abc212", "arc212"].contains(&problem.contest_id.as_str()));
    assert!(800.0 <= problem.difficulty && problem.difficulty <= 1000.0);
}

#[tokio::test]
async fn test_multiple_contests_with_difficulty_range_narrows_candidates() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=950&max=975&contest=abc,arc&contest_from=212&contest_to=212",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
        difficulty: f64,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert_eq!(problem.id, "arc212_a");
    assert_eq!(problem.contest_id, "arc212");
    assert_eq!(problem.difficulty, 950.0);
}

#[tokio::test]
async fn test_contest_and_round_filters_exclude_out_of_range_contests() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=800&max=1000&contest=abc&contest_from=213&contest_to=213",
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert_eq!(problem.id, "abc213_a");
    assert_eq!(problem.contest_id, "abc213");
}

#[tokio::test]
async fn test_other_contest_filter() {
    let (status, body) = build_and_send(Method::GET, "/?min=0&max=1500&contest=other").await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        contest_id: String,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert_eq!(problem.contest_id, "typical90");
}

#[tokio::test]
async fn test_contest_number_filter_excludes_other_contests() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=0&max=1500&contest=other&contest_from=90",
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    #[derive(serde::Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let err: ErrorResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(err.message, "指定Diff範囲に該当する問題がありませんでした");
}

#[tokio::test]
async fn test_contest_from_greater_than_to() {
    let (status, body) = build_and_send(Method::GET, "/?contest_from=300&contest_to=200").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'contest_from' cannot be greater than 'contest_to'.");
}

#[tokio::test]
async fn test_contest_range_includes_problem_without_difficulty_when_range_omitted() {
    let (status, body) =
        build_and_send(Method::GET, "/?contest=abc&contest_from=459&contest_to=459").await;
    assert_eq!(status, StatusCode::OK);

    #[derive(serde::Deserialize)]
    struct ProblemResponse {
        id: String,
        contest_id: String,
        difficulty: Option<f64>,
    }

    let problem: ProblemResponse = serde_json::from_str(&body).unwrap();

    assert_eq!(problem.id, "abc459_a");
    assert_eq!(problem.contest_id, "abc459");
    assert_eq!(problem.difficulty, None);
}

#[tokio::test]
async fn test_full_difficulty_range_excludes_problem_without_difficulty() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=0&max=3854&contest=abc&contest_from=459&contest_to=459",
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    #[derive(serde::Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let err: ErrorResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(err.message, "指定Diff範囲に該当する問題がありませんでした");
}

#[tokio::test]
async fn test_explicit_difficulty_range_excludes_problem_without_difficulty() {
    let (status, body) = build_and_send(
        Method::GET,
        "/?min=0&max=1500&contest=abc&contest_from=459&contest_to=459",
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    #[derive(serde::Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let err: ErrorResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(err.message, "指定Diff範囲に該当する問題がありませんでした");
}
