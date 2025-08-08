use hyper::service::{make_service_fn, service_fn};
use hyper::{Server};
use std::net::SocketAddr;
use std::convert::Infallible;

use backend::router;

#[tokio::main]
async fn main() {
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  let make_svc = make_service_fn(|_conn| async {
    Ok::<_, Infallible>(service_fn(router))
  });

  let server = Server::bind(&addr).serve(make_svc);

  println!("Running server on : http://{}.", addr);

  if let Err(e) = server.await {
    eprintln!("error on {}.", e);
  } else {
    println!("error was shut down.");
  }
}