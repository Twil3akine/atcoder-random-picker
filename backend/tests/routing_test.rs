#![allow(unused)]

use backend::utils::routing::{router, AppState};
use backend::utils::api::{Problem, ProblemModel};
use hyper::{Body, Method, Request, StatusCode};
use std::assert;
use std::collections::HashMap;
use std::sync::Arc;

fn build_test_state() -> Arc<AppState> {
    let mut problem_models = HashMap::new();
    problem_models.insert(
        "abc001_a".to_string(),
        ProblemModel { difficulty: Some(1000.0) },
    );

    let problems = vec![Problem {
        id: "abc001_a".to_string(),
        contest_id: "abc001".to_string(),
        name: "A - Test Problem".to_string(),
    }];

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
    let (status, body) = build_and_send(Method::GET, "/?under=0&over=500").await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    #[derive(serde::Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    let err: ErrorResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(err.message, "指定Diff範囲に該当する問題がありませんでした");
}

#[tokio::test]
async fn test_under_greater_than_over() {
    let (status, body) = build_and_send(Method::GET, "/?under=1500&over=500").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'under' cannot bt greater than 'over'.");
}

#[tokio::test]
async fn test_random_range() {
    let (status, body) = build_and_send(Method::GET, "/?under=500&over=1500").await;
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