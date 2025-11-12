mod error;
mod handlers;
mod helper;
mod middleware;

use handlers::*;

use std::sync::Arc;

use auth_service::AuthService;
use axum::http::HeaderValue;
use axum::routing::get;
use axum::{routing::post, Router};
use axum_cookie::{CookieLayer, CookieManager};
use db::Db;
use http::{HeaderName, Method};
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

pub use crate::error::{Error, Result};
pub use crate::middleware::AuthLayer;

#[derive(Clone)]
struct UnauthenticatedSharedState {
    db_repo: Arc<Db>,
    auth_service: Arc<AuthService>,
}

#[derive(Clone)]
struct AuthenticatedSharedState {
    pub db_repo: Arc<Db>,
    pub auth_service: Arc<AuthService>,
}

pub struct RouterService {
    router: Router,
    tcp_listener: TcpListener,
}

impl RouterService {
    pub async fn init(db: Arc<Db>, auth_service: Arc<AuthService>) -> Result<RouterService> {

        let app =Self::get_routes(db, auth_service);
        let listener = TcpListener::bind("localhost:8084").await?;
        let router_service = Self {
            router: app,
            tcp_listener: listener,
        };

        Ok(router_service)
    }

    fn unauthenticated_routes(db: Arc<Db>, auth_service: Arc<AuthService>) -> Router {
        let unauthenticated_state = UnauthenticatedSharedState {
            db_repo: db.clone(),
            auth_service: auth_service.clone(),
        };
        let authenticated_state = AuthenticatedSharedState {
            db_repo: db,
            auth_service,
        };
        let cors_layer = CorsLayer::new()
            .allow_credentials(true)
            .allow_methods([Method::GET,Method::POST])
            .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
                "http://localhost:3000",
            )))
            .allow_headers([http::header::CONTENT_TYPE,http::header::AUTHORIZATION]);

        let app = Router::new()
            .route("/blog/publish", post(create_blog))
            .with_state(authenticated_state.clone())
            .layer(AuthLayer::new(authenticated_state.clone()))
            .route("/auth/session", post(session))
            .with_state(unauthenticated_state.clone())
            .route("/auth/refresh_token", get(refresh_token))
            .layer(CookieLayer::default())
            .with_state(unauthenticated_state)
            .layer(cors_layer);

        app
    }

    fn get_routes(db: Arc<Db>, auth_service: Arc<AuthService>) -> Router {
        Router::new().merge(Self::unauthenticated_routes(db, auth_service))
    }

    pub async fn run(self) -> Result<()> {
        axum::serve(self.tcp_listener, self.router).await?;
        Ok(())
    }
}
