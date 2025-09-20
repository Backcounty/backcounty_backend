mod error;
mod handlers;

use handlers::*;

use std::sync::Arc;

use auth_service::AuthService;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::{routing::post, Router};
use db::Db;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub use crate::error::{Error, Result};


#[derive(Clone)]
struct UnauthenticatedSharedState {
    db_repo: Arc<Db>,
    auth_service: Arc<AuthService>,
}

pub struct RouterService {
    router: Router,
    tcp_listener: TcpListener,
}

impl RouterService {
    pub async fn init(db: Arc<Db>, auth_service: Arc<AuthService>) -> Result<RouterService> {
        let cors = CorsLayer::new()
            .allow_methods([Method::POST])
            .allow_origin([HeaderValue::from_static("http://localhost:3000")])
            .allow_headers([CONTENT_TYPE]);

        let app = Router::from(Self::get_routes(db, auth_service)).layer(cors);
        let listener = TcpListener::bind("localhost:8084").await?;
        let router_service = Self {
            router: app,
            tcp_listener: listener,
        };

        Ok(router_service)
    }

    fn get_routes(db: Arc<Db>, auth_service: Arc<AuthService>) -> Router {
        Router::new().merge(Self::unauthenticated_routes(db, auth_service))
    }

    fn unauthenticated_routes(db: Arc<Db>, auth_service: Arc<AuthService>) -> Router {
        let shared_state = UnauthenticatedSharedState {
            db_repo: db,
            auth_service,
        };
        let app = Router::new()
            .route("/auth/session", post(session))
            .with_state(shared_state);
        app
    }

    pub async fn run(self) -> Result<()> {
        axum::serve(self.tcp_listener, self.router).await?;
        Ok(())
    }
}
