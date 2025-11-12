use auth_service::TokenPair;
use axum::response::IntoResponse;
use axum::Json;
use axum_cookie::cookie::CookieBuilder;
use axum_cookie::prelude::SameSite;
use http::{HeaderMap, HeaderValue};
use serde_json::json;

use crate::error::Result;
pub(crate) fn build_token_pair_response(token_pair: TokenPair) -> Result<impl IntoResponse> {
    //Builds Cookie
    let refresh_token_cookie = CookieBuilder::new("refresh_token", token_pair.refresh_token.0)
        .path("/")
        .same_site(SameSite::Strict) // or None if cross-site
        .build();
    let mut headers = HeaderMap::new();

    headers.insert(
        "Set-Cookie",
        HeaderValue::from_str(&refresh_token_cookie.to_string())?,
    );

    headers.insert(
        "Access-Control-Allow-Credentials",
        HeaderValue::from_str("true")?,
    );

    Ok((
        headers,
        Json(json!({"access_token": token_pair.access_token.0.to_string()})),
    ))
}
