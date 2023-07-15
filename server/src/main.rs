use std::fs;

use axum::{
    response::{Html, IntoResponse},
    routing::{delete, get, options, post, put},
    Router, Server,
};

use hyper::{Method, StatusCode};
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let app = Router::new()
        .route("/", get(introduction))
        .route("/", options(|| async { StatusCode::NO_CONTENT }))
        .route("/:key", put(routes::boards_routes::put_board))
        .route("/:key", get(routes::boards_routes::get_board))
        .route(
            "/denied/:key",
            put(routes::deny_list_routes::add_denied_key),
        )
        .route(
            "/denied/:key",
            delete(routes::deny_list_routes::delete_denied_key),
        )
        .route("/denied", get(routes::deny_list_routes::get_denied_keys))
        .route("/auth", post(routes::auth_routes::sign_up))
        .route("/auth/login", post(routes::auth_routes::sign_in))
        .route("/auth", delete(routes::auth_routes::remove_user))
        .route("/auth/keys", put(routes::auth_routes::assign_keys_to_user))
        .route(
            "/auth/keys",
            post(routes::auth_routes::get_user_assigned_keys),
        )
        .route("/auth/keys", delete(routes::auth_routes::remove_user_keys))
        .route(
            "/boards/feed",
            get(routes::boards_routes::get_followed_boards),
        )
        .route(
            "/boards/following/:key",
            put(routes::auth_routes::follow_key),
        )
        .route(
            "/boards/following/:key",
            delete(routes::auth_routes::unfollow_key),
        )
        .route(
            "/boards/following",
            get(routes::auth_routes::get_followed_keys),
        )
        .route("/boards", get(routes::boards_routes::get_recent_boards))
        .route(
            "/boards/count",
            get(routes::boards_routes::get_boards_count),
        )
        // cors
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                    Method::POST,
                ])
                .allow_origin(Any)
                .allow_headers(Any),
        );
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

// async fn propagate_header<B>(req: Request<B>, next: Next<B>) -> Response {
//     if req.method() == Method::OPTIONS {
//         let mut res = next.run(req).await;
//         res.headers_mut().insert(
//             "Access-Control-Allow-Methods",
//             "GET, OPTIONS, PUT".parse().unwrap(),
//         );
//         res.headers_mut()
//             .insert("Access-Control-Allow-Origin", "*".parse().unwrap());
//         res.headers_mut().insert(
//             "Access-Control-Allow-Headers",
//             "Content-Type, If-Modified-Since, Spring-Signature, Spring-Version"
//                 .parse()
//                 .unwrap(),
//         );
//         res.headers_mut().insert(
//             "Access-Control-Expose-Headers",
//             "Content-Type, Last-Modified, Spring-Signature, Spring-Version"
//                 .parse()
//                 .unwrap(),
//         );
//         return (StatusCode::NO_CONTENT).into_response();
//     }
//     let res = next.run(req).await;
//     res
// }
