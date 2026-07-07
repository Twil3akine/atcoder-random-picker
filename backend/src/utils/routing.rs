use chrono::{DateTime, Local};
use core::prelude::v1::derive;
use hyper::{header, Body, Request, Response, StatusCode};
use rand::seq::IteratorRandom;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::{From, Infallible};
use std::iter::Iterator;
use std::option::Option::None;
use std::result::Result::Ok;
use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use crate::utils::api::{Problem, ProblemModel};

const MIN_DIFFICULTY: f64 = 0.0;
const MAX_DIFFICULTY: f64 = 3854.0;

#[derive(Clone)]
pub struct AppState {
    pub problems: Vec<Problem>,
    pub problem_models: HashMap<String, ProblemModel>,
}

#[derive(Serialize)]
enum Contest {
    ABC,
    ARC,
    AGC,
    Other,
    Prefix(String),
}

impl Contest {
    fn from_id(id: &str) -> Self {
        if id.starts_with("abc") {
            Contest::ABC
        } else if id.starts_with("arc") {
            Contest::ARC
        } else if id.starts_with("agc") {
            Contest::AGC
        } else if id == "other" || id == "others" {
            Contest::Other
        } else {
            Contest::Prefix(id.to_string())
        }
    }
}

#[derive(Serialize)]
struct ProblemResponse {
    id: String,
    contest_id: String,
    name: String,
    difficulty: Option<f64>,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

fn bad_request(message: &str) -> Response<Body> {
    let mut res = Response::new(Body::from(message.to_string()));
    *res.status_mut() = StatusCode::BAD_REQUEST;
    with_cors_headers(res)
}

async fn get_parameter(req: &Request<Body>) -> HashMap<String, String> {
    let query = req.uri().query().unwrap_or("");
    url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect()
}

fn parse_optional_f64(params: &HashMap<String, String>, key: &str) -> Result<Option<f64>, String> {
    params
        .get(key)
        .map(|value| {
            value
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite())
                .ok_or_else(|| format!("'{}' must be a number.", key))
        })
        .transpose()
}

fn parse_optional_u32(params: &HashMap<String, String>, key: &str) -> Result<Option<u32>, String> {
    params
        .get(key)
        .map(|value| {
            value
                .parse::<u32>()
                .map_err(|_| format!("'{}' must be a positive integer.", key))
        })
        .transpose()
}

fn log(now: DateTime<Local>, method: &str, path: &str, status: StatusCode) {
    println!(
        "[{}] {} {} -> {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        method,
        path,
        status
    );
}

fn with_cors_headers(mut res: Response<Body>) -> Response<Body> {
    let headers = res.headers_mut();

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, OPTIONS".parse().unwrap(),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Content-Type".parse().unwrap(),
    );

    res
}

fn standard_contest_number(contest_id: &str) -> Option<u32> {
    for prefix in ["abc", "arc", "agc"] {
        if let Some(number) = contest_id.strip_prefix(prefix) {
            return number.parse().ok();
        }
    }

    None
}

pub async fn router(
    req: Request<Body>,
    state: Arc<AppState>,
) -> Result<Response<Body>, Infallible> {
    let now = Local::now();
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    let res = match (req.method(), req.uri().path()) {
        (&hyper::Method::OPTIONS, _) => {
            let res = Response::new(Body::empty());
            Ok(with_cors_headers(res))
        }

        (&hyper::Method::GET, "/") => {
            let params: HashMap<String, String> = get_parameter(&req).await;

            let min: f64 = match parse_optional_f64(&params, "min") {
                Ok(min) => min.unwrap_or(MIN_DIFFICULTY),
                Err(message) => return Ok(bad_request(&message)),
            };
            let max: f64 = match parse_optional_f64(&params, "max") {
                Ok(max) => max.unwrap_or(MAX_DIFFICULTY),
                Err(message) => return Ok(bad_request(&message)),
            };
            let allows_unknown_difficulty =
                !params.contains_key("min") && !params.contains_key("max");

            let contest_filters: Vec<Contest> = params
                .get("contest")
                .map(|s| {
                    s.split(',')
                        .filter_map(|id| {
                            let id = id.trim();
                            (!id.is_empty()).then(|| Contest::from_id(id))
                        })
                        .collect::<Vec<Contest>>()
                })
                .unwrap_or_default();

            let contest_from: Option<u32> = match parse_optional_u32(&params, "contest_from") {
                Ok(contest_from) => contest_from,
                Err(message) => return Ok(bad_request(&message)),
            };
            let contest_to: Option<u32> = match parse_optional_u32(&params, "contest_to") {
                Ok(contest_to) => contest_to,
                Err(message) => return Ok(bad_request(&message)),
            };

            if min > max {
                return Ok(bad_request("'min' cannot be greater than 'max'."));
            }

            if min < MIN_DIFFICULTY {
                return Ok(bad_request("'min' cannot be less than 0."));
            }

            if max > MAX_DIFFICULTY {
                return Ok(bad_request("'max' cannot be greater than 3854."));
            }

            if contest_from
                .zip(contest_to)
                .is_some_and(|(from, to)| from > to)
            {
                return Ok(bad_request(
                    "'contest_from' cannot be greater than 'contest_to'.",
                ));
            }

            let mut rng = rand::thread_rng();

            let selected = state
                .problems
                .iter()
                .filter(|p| {
                    if contest_filters.is_empty() {
                        return true;
                    }
                    contest_filters.iter().any(|filter| match filter {
                        Contest::ABC => p.contest_id.starts_with("abc"),
                        Contest::ARC => p.contest_id.starts_with("arc"),
                        Contest::AGC => p.contest_id.starts_with("agc"),
                        Contest::Other => {
                            !p.contest_id.starts_with("abc")
                                && !p.contest_id.starts_with("arc")
                                && !p.contest_id.starts_with("agc")
                        }
                        Contest::Prefix(s) => p.contest_id.starts_with(s),
                    })
                })
                .filter(|p| {
                    let Some(number) = standard_contest_number(&p.contest_id) else {
                        return contest_from.is_none() && contest_to.is_none();
                    };

                    contest_from.is_none_or(|from| number >= from)
                        && contest_to.is_none_or(|to| number <= to)
                })
                .filter_map(|p| {
                    state
                        .problem_models
                        .get(&p.id)
                        .and_then(|m| match m.difficulty {
                            Some(diff) if min <= diff && diff <= max => Some(ProblemResponse {
                                id: p.id.clone(),
                                contest_id: p.contest_id.clone(),
                                name: p.name.clone(),
                                difficulty: Some(diff),
                            }),
                            None if allows_unknown_difficulty => Some(ProblemResponse {
                                id: p.id.clone(),
                                contest_id: p.contest_id.clone(),
                                name: p.name.clone(),
                                difficulty: None,
                            }),
                            _ => None,
                        })
                })
                .choose(&mut rng);

            match selected {
                Some(problem) => {
                    let body = serde_json::to_string(&problem).unwrap();
                    Ok(with_cors_headers(Response::new(Body::from(body))))
                }
                None => {
                    let error_body = serde_json::to_string(&ErrorResponse {
                        message: "指定Diff範囲に該当する問題がありませんでした".to_string(),
                    })
                    .unwrap();

                    let mut not_found = Response::new(Body::from(error_body));
                    *not_found.status_mut() = StatusCode::NOT_FOUND;
                    not_found
                        .headers_mut()
                        .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

                    Ok(with_cors_headers(not_found))
                }
            }
        }

        _ => {
            let mut not_found = Response::new(Body::from("404 Not Found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(with_cors_headers(not_found))
        }
    };

    let status = res
        .as_ref()
        .map(|r| r.status())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    log(now, &method, &path, status);

    res
}
