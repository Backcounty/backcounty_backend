mod handlers;
mod error;
mod constants;

use handlers::*;
use error::*;

use axum::http::{HeaderValue, Method};
use axum::{routing::post, Router};
use axum::http::header::CONTENT_TYPE;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer};

use crate::handle_oauth;

pub(crate) fn unauthenticated_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin([HeaderValue::from_static("http://localhost:3000")])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/oauth", post(handle_oauth))
        .layer(cors);
    app
}

pub async fn run() {
    let listener = TcpListener::bind("localhost:8084").await.unwrap();
    let _=axum::serve(listener, unauthenticated_routes()).await;
}
