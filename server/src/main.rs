use std::{fs};

use axum::{
    middleware::Next,
    response::{Html, IntoResponse, Response},
    routing::{get, options, put},
    Router, Server,
};

use hyper::{Method, Request, StatusCode};

mod services;
mod routes;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let app = Router::new()
                        .route("/", get(introduction))
                        .route("/", options(|| async { StatusCode::NO_CONTENT }))
                        .route("/:key", put(routes::boards_routes::put_board))
                        .layer(axum::middleware::from_fn(propagate_header));
    // Start the server
    let addr = ("127.0.0.1:".to_string() + &port).parse().unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

async fn introduction() -> impl IntoResponse {
    let contents = fs::read_to_string("src/assets/introduction.html").expect(
        "
    Something went wrong reading the file",
    );
    Html(contents)
}

async fn propagate_header<B>(req: Request<B>, next: Next<B>) -> Response {
    print!("{:?}", req.headers() );
    if req.method() == Method::OPTIONS {
        let mut res = next.run(req).await;
        res.headers_mut().insert("Access-Control-Allow-Methods", "GET, OPTIONS, PUT".parse().unwrap());
        res.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());
        res.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, If-Modified-Since, Spring-Signature, Spring-Version".parse().unwrap());
        res.headers_mut().insert("Access-Control-Expose-Headers", "Content-Type, Last-Modified, Spring-Signature, Spring-Version".parse().unwrap()); 
        return (StatusCode::NO_CONTENT).into_response();
    } 
    let res = next.run(req).await;
    res
}
   
