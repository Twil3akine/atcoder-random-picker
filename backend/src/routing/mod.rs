use hyper::{Body, Request, Response, StatusCode, header};
use std::convert::{From, Infallible};
use std::collections::HashMap;
use std::format;
use std::option::Option::None;
use std::result::Result::Ok;
use url::form_urlencoded;
use chrono::{DateTime, Local};
use rand::Rng;

async fn get_parameter(req: Request<Body>) -> HashMap<String, String> {
  let query = req.uri().query().unwrap_or("");
  let params: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
    .into_owned()
    .collect();

  params
}

fn get_random_number(under: u32, over: u32) -> u32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(under..=over)
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

pub async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let now= Local::now();

  let path = req.uri().path().to_string();
  let method = req.method().to_string();

  let res = match (req.method(), req.uri().path()) {
      (&hyper::Method::OPTIONS, _) => {
        let res = Response::new(Body::empty());
        Ok(with_cors_headers(res))
      }

      (&hyper::Method::GET, "/") => {
        let params: HashMap<String, String> = get_parameter(req).await;
        
        let under: u32 = match params.get("under").and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => 0,
        };

        let over: u32 = match params.get("over").and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => 3854,
        };

        if under > over {
          let mut bad_request = Response::new(Body::from("'under' cannot bt greater than 'over'."));
          *bad_request.status_mut() = StatusCode::BAD_REQUEST;
          return Ok(bad_request);
        }

        let random_number = get_random_number(under, over);
        let response_body = format!("{}", random_number);
        Ok(with_cors_headers(Response::new(Body::from(response_body))))
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