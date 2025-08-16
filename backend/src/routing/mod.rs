use hyper::{Body, Request, Response, StatusCode, header};
use std::option::Option::None;
use std::result::Result::Ok;
use std::{collections::HashMap};
use std::convert::{From, Infallible};
use std::sync::Arc;
use chrono::{DateTime, Local};
use rand;
use rand::seq::IteratorRandom;
use serde::Serialize;

use crate::api::{Problem, ProblemModel};

#[derive(Clone)]
pub struct AppState {
  pub problems: Vec<Problem>,
  pub problem_models: HashMap<String, ProblemModel>,
}

#[derive(Serialize)]
struct ProblemResponse {
  id: String,
  contest_id: String,
  name: String,
  difficulty: Option<f64>,
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
        
        let under: f64 = params.get("under").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let over: f64 = params.get("over").and_then(|s| s.parse().ok()).unwrap_or(3854.0);

        if under > over {
          let mut bad_request = Response::new(Body::from("'under' cannot bt greater than 'over'."));
          *bad_request.status_mut() = StatusCode::BAD_REQUEST;
          return Ok(bad_request);
        }

        let mut rng = rand::thread_rng();
        let selected = state.problems.iter().filter_map(|p| {
          state.problem_models.get(&p.id).and_then(|m| {
            m.difficulty.and_then(|diff| {
              if under as i32 <= diff as i32 && diff as i32 <= over as i32 {
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
        }).choose(&mut rng);

        match selected {
          Some(problem) => {
            let body = serde_json::to_string(&problem).unwrap();
            Ok(with_cors_headers(Response::new(Body::from(body))))
          }
          None => {
            let mut not_found = Response::new(Body::from("No problem found in given range."));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
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