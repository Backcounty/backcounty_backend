mod constants;
mod error;
mod handlers;

pub use crate::error::{Error, Result};
use handlers::*;
use std::sync::Arc;

use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::{routing::post, Router};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::handle_oauth;
use db::Db;

#[derive(Clone)]
struct UnauthenticatedSharedState {
    db_repo: Arc<Db>,
}

pub struct RouterService {
    router: Router,
    tcp_listener: TcpListener,
}

impl RouterService {
    pub async fn init(db: Arc<Db>) -> Result<RouterService> {
        let cors = CorsLayer::new()
            .allow_methods([Method::POST])
            .allow_origin([HeaderValue::from_static("http://localhost:3000")])
            .allow_headers([CONTENT_TYPE]);

        let app = Router::from(Self::get_routes(db)).layer(cors);
        let listener = TcpListener::bind("localhost:8084").await?;
        let router_service = Self {
            router: app,
            tcp_listener: listener,
        };

        Ok(router_service)
    }
    fn get_routes(db: Arc<Db>) -> Router {
        Router::new().merge(Self::unauthenticated_routes(db))
    }

    fn unauthenticated_routes(db: Arc<Db>) -> Router {
        let shared_state = UnauthenticatedSharedState { db_repo: db };
        let app = Router::new()
            .route("/oauth", post(handle_oauth))
            .with_state(shared_state);
        app
    }

    pub async fn run(self)->Result<()> {
        axum::serve(self.tcp_listener, self.router).await?;
        Ok(())
    }
}
