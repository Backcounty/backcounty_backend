use crate::error::Error;
use crate::{AuthenticatedSharedState, Result};
use auth_service::{JwtService, UserInfo};
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use db::Db;
use headers::{self, authorization::Bearer, Authorization, Header};
use http::header::AUTHORIZATION;
use http::{HeaderValue, StatusCode};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::layer::Layer;
use tower::Service;
use axum::Extension;

#[derive(Clone)]
pub struct AuthMiddlewareService<T> {
    inner: T,
    auth_state: AuthenticatedSharedState,
}

#[derive(Debug,Clone)]
pub struct Ctx{
    pub user_info: UserInfo
}
impl<S> Service<Request> for AuthMiddlewareService<S>
where
    S: Service<Request, Response = Response> + Clone,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = core::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<core::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        // println!("Header values:{:?}",);
        let authorization=req.headers().get(http::header::AUTHORIZATION);
        if authorization.is_none(){
            return Box::pin(async {
                return Ok((StatusCode::BAD_REQUEST,"Authorization Header Required").into_response());
            });
        }
        let authorization_str =authorization.expect("Should exists").to_owned().to_str().map(|x|x.to_string());
        if let Err(e)= authorization_str {
            return Box::pin(async {
                return Ok((StatusCode::INTERNAL_SERVER_ERROR,"Error Parsing Authorization Header to String").into_response());
            })
        }
        let  authorization_str =authorization_str.expect("Should exists").strip_prefix("Bearer ").map(|x|x.to_string());
        if authorization_str.is_none(){
            return Box::pin(async {
                return Ok((StatusCode::INTERNAL_SERVER_ERROR,"Should Contain Bearer Token").into_response());
            })
        }
        let bearer_token=authorization_str.expect("Should contains it");

        let authenticated_user_claims =self.auth_state.auth_service.jwt_service.validate_access_token(&bearer_token);
        if let Err(e)= authenticated_user_claims {
            return Box::pin(async {
                return Ok((StatusCode::UNAUTHORIZED,Error::Other(e.into())).into_response());

            });
        }
        let user_info:UserInfo=authenticated_user_claims.expect("Should Be Infallible").into();

        let mut req=req;
        req.extensions_mut().insert(Ctx{user_info});
        let future = self.inner.call(req);
        Box::pin(async move { future.await })
    }
}

#[derive(Clone)]
pub struct AuthLayer {
    auth_state: AuthenticatedSharedState,
}
impl AuthLayer {
    pub fn new(auth_state: AuthenticatedSharedState) -> Self {
        AuthLayer { auth_state }
    }
}
impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddlewareService<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthMiddlewareService {
            inner: service,
            auth_state: self.auth_state.clone(),
        }
    }
}
