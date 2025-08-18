use backend::{api, router, AppState};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::net::SocketAddr;
use std::convert::{From, Infallible};
use std::{println};
use std::result::Result::{Err, Ok};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    match api::fetch_problem().await {
        Ok((problems, problem_models)) => {
            println!("Succeeded to fetch problems");

            let state = Arc::new(AppState { problems, problem_models });

            // Fly.io 環境変数 PORT を使用
            let port: u16 = std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number");

            // 0.0.0.0 でバインド
            let addr = SocketAddr::from(([0, 0, 0, 0], port));

            let make_svc = make_service_fn(move |_conn| {
                let state = state.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        let state = state.clone();
                        router(req, state)
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