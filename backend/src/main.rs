use backend::utils::api;
use backend::utils::ratelimiter::RateLimiter;
use backend::utils::routing::{AppState, router};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Server, Body, Response, StatusCode};
use std::sync::Arc;
use std::net::SocketAddr;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    match api::fetch_problem().await {
        Ok((problems, problem_models)) => {
            println!("Succeeded to fetch problems");

            let state = Arc::new(AppState { problems, problem_models });
            let limiter = RateLimiter::new();

            // Fly.io 環境変数 PORT を使用
            let port: u16 = std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number");

            // 0.0.0.0 でバインド
            let addr = SocketAddr::from(([0, 0, 0, 0], port));

            let make_svc = make_service_fn(move |conn: &hyper::server::conn::AddrStream| {
                let remote_addr = conn.remote_addr().ip();
                let state = state.clone();
                let limiter = limiter.clone();

                async move {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        let state = state.clone();
                        let limiter = limiter.clone();
                        let ip = remote_addr;

                        async move {
                            if !limiter.check(ip).await {
                                let mut res = Response::new(Body::from("Too Many Requests"));
                                *res.status_mut() = StatusCode::TOO_MANY_REQUESTS;
                                return Ok::<_, Infallible>(res)
                            }
                            router(req, state).await
                        }
                    }))
                }
            });

            let server = Server::bind(&addr).serve(make_svc);

            println!("Running server on http://{}.", addr);

            if let Err(e) = server.await {
                eprintln!("error on {}.", e);
            } else {
                println!("server shut down.");
            }
        }
        Err(e) => eprintln!("Failed to fetch problems: {}", e),
    }
}