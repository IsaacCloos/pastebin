use std::convert::Infallible;

use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

#[tokio::main]
async fn main() {
    let app = make_service_fn(move |_stream: &AddrStream| async move {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    Server::bind(&([127, 0, 0, 1], 8888).into())
        .serve(app)
        .await
        .unwrap();
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .body(Body::from("Hello World!\n"))
        .unwrap())
}