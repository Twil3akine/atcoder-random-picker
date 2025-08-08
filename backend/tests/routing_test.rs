#![allow(unused_comparisons)]

use backend::router;
use hyper::{Body, Method, Request, StatusCode};

async fn build_and_send(method: Method, path: &str) -> (StatusCode, String) {
	let req = Request::builder()
		.method(method)
		.uri(path)
		.body(Body::empty())
		.unwrap();

	let res = router(req).await.unwrap();

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
async fn test_calc_with_both_params() {
    let (status, body) = build_and_send(Method::GET, "/?under=10&over=20").await;

    assert_eq!(status, StatusCode::OK);
	let random_number: u32 = body.parse().unwrap();
    assert!(10 <= random_number && random_number <= 20);
}

#[tokio::test]
async fn test_calc_without_under() {
    let (status, body) = build_and_send(Method::GET, "/?over=5").await;

    assert_eq!(status, StatusCode::OK);
    let random_number: u32 = body.parse().unwrap();
    assert!(0 <= random_number && random_number <= 5);
}

#[tokio::test]
async fn test_calc_without_over() {
    let (status, body) = build_and_send(Method::GET, "/?under=100").await;

    assert_eq!(status, StatusCode::OK);
	let random_number: u32 = body.parse().unwrap();
    assert!(100 <= random_number && random_number <= 3854);
}

#[tokio::test]
async fn test_calc_without_any_params() {
    let (status, body) = build_and_send(Method::GET, "/").await;

    assert_eq!(status, StatusCode::OK);
    let random_number: u32 = body.parse().unwrap();
    assert!(0 <= random_number && random_number <= 3854);
}

#[tokio::test]
async fn test_calc_under_greater_than_over() {
    let (status, body) = build_and_send(Method::GET, "/?under=50&over=10").await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body, "'under' cannot bt greater than 'over'.");
}