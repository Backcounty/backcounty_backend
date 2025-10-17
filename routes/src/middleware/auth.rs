use crate::error::Error;
use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;
use tower::layer::Layer;

#[derive(Clone)]
pub struct AuthService<T> {
    inner: T,
}

impl<S> Service<Request> for AuthService<S>
where
    S: Service<Request, Response = Response> + Clone,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let access_token_header = req.headers().get("x-access-token");

        match access_token_header {
            Some(access_token_header_value) if !access_token_header_value.is_empty() => {
                // Valid token present, forward to inner service
                let future = self.inner.call(req);
                Box::pin(async move {
                    future.await
                })
            }
            Some(_) => {
                // Empty token
                Box::pin(async move {
                    Ok(Error::Custom("Empty Access Token".into()).into_response())
                })
            }
            None => {
                // No token header
                Box::pin(async move {
                    Ok(Error::Custom("x-access-token header required".into()).into_response())
                })
            }
        }
    }
}

#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthService { inner: service }
    }
}