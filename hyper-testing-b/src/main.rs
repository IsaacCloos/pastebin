use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::convert::Infallible;
use tracing::{info, instrument};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::FULL)
        .with_target(false)
        .with_level(false)
        .init();

    let app = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let local_addr: std::net::SocketAddr = ([127, 0, 0, 1], 8888).into();
    info!("listening on http://{}", local_addr);

    Server::bind(&local_addr).serve(app).await.unwrap();
}

#[instrument(skip_all)]
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .body(Body::from("Hello World!\n"))
        .unwrap())
}
