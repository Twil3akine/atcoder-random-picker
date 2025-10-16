use hyper::{Body, Request, Response, StatusCode, header};
use core::prelude::v1::derive;
use std::iter::Iterator;
use std::option::Option::None;
use std::result::Result::Ok;
use std::string::{String, ToString};
use std::vec::Vec;
use std::{collections::HashMap};
use std::convert::{From, Infallible};
use std::sync::Arc;
use chrono::{DateTime, Local};
use rand::seq::IteratorRandom;
use serde::Serialize;

use crate::utils::api::{Problem, ProblemModel};

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
  Other(String),
}

impl Contest {
  fn from_id(id: &str) -> Self {
    if id.starts_with("abc") {
      Contest::ABC
    } else if id.starts_with("arc") {
      Contest::ARC
    } else if id.starts_with("agc") {
      Contest::AGC
    } else {
      Contest::Other(id.to_string())
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

async fn get_parameter(req: &Request<Body>) -> HashMap<String, String> {
  let query = req.uri().query().unwrap_or("");
  url::form_urlencoded::parse(query.as_bytes())
    .into_owned()
    .collect()
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
  headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS".parse().unwrap());
  headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type".parse().unwrap());

  res
}

pub async fn router(req: Request<Body>, state: Arc<AppState>) -> Result<Response<Body>, Infallible> {
  let now= Local::now();
  let path = req.uri().path().to_string();
  let method = req.method().to_string();

  let res = match (req.method(), req.uri().path()) {
      (&hyper::Method::OPTIONS, _) => {
        let res = Response::new(Body::empty());
        Ok(with_cors_headers(res))
      }

      (&hyper::Method::GET, "/") => {
        let params: HashMap<String, String> = get_parameter(&req).await;
        
        let min: f64 = params.get("min").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let max: f64 = params.get("max").and_then(|s| s.parse().ok()).unwrap_or(3854.0);

        let contest_filters: Vec<Contest> = params
          .get("contest")
          .map(|s| {
            s.split(',')
              .map(|id| Contest::from_id(id.trim()))
              .collect::<Vec<Contest>>()
          })
          .unwrap_or_default();

        if min > max {
          let mut bad_request = Response::new(Body::from("'min' cannot bt greater than 'max'."));
          *bad_request.status_mut() = StatusCode::BAD_REQUEST;
          return Ok(bad_request);
        }

        let mut rng = rand::thread_rng();

        let selected = state.problems
          .iter()
          .filter(|p| {
            if contest_filters.is_empty() {
              return true;
            }
            contest_filters.iter().any(|filter| match filter {
              Contest::ABC => p.contest_id.starts_with("abc"),
              Contest::ARC => p.contest_id.starts_with("arc"),
              Contest::AGC => p.contest_id.starts_with("agc"),
              Contest::Other(s) => p.contest_id.starts_with(s),
            })
          })
          .filter_map(|p| {
            state.problem_models.get(&p.id).and_then(|m| {
              m.difficulty.and_then(|diff| {
                if min <= diff && diff <= max {
                  Some(ProblemResponse {
                    id: p.id.clone(),
                    contest_id: p.contest_id.clone(),
                    name: p.name.clone(),
                    difficulty: Some(diff),
                  })
                } else {
                  None
                }
              })
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
            }).unwrap();

            let mut not_found = Response::new(Body::from(error_body));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            not_found.headers_mut().insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

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

  let status = res.as_ref().map(|r| r.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

  log(now, &method, &path, status);

  res
}