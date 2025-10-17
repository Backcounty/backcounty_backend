use cookie::time::OffsetDateTime;
use auth_service::{JwtService, TokenPair};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};
use axum::Json;
use cookie::Cookie;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use chrono::Duration;

use super::super::UnauthenticatedSharedState;
use crate::Result;
#[derive(Debug, Deserialize)]
pub struct AuthorizationCode {
    pub code: String,
}

pub async fn session(
    State(state): State<UnauthenticatedSharedState>,
    authorization_code: String,
) -> Result<Response> {
    let authorization_code_struct =
        serde_json::from_str::<AuthorizationCode>(authorization_code.as_str())?;
    let token_pair=create_new_session(&state, &authorization_code_struct).await?;


    let mut headers = HeaderMap::new();
    //Cookie header to store refresh token and device id
    let now = Utc::now().timestamp() ;
    let exp = now.saturating_add(Duration::days(15).num_seconds());
    let exp=OffsetDateTime::from_unix_timestamp(exp)?;
    let refresh_token_cookie = Cookie::build(("refresh_token", token_pair.refresh_token.0))
        .http_only(true)
        .expires(exp)
        .path("/blog")
        .build();

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
    )
        .into_response())
}

async fn create_new_session(
    state: &UnauthenticatedSharedState,
    authorization_code: &AuthorizationCode,
) -> Result<TokenPair> {
    let user_profile = state
        .auth_service
        .get_profile_from_code(&authorization_code.code)
        .await?;

    // Checks if a user exists, if not create user
    let user_id = if let Some(user) = state
        .db_repo
        .as_ref()
        .user_repo
        .get_user_from_email(&user_profile.email)
        .await?
    {
        user.user_id
    } else {
        let user_id = state
            .db_repo
            .as_ref()
            .user_repo
            .create_user(&user_profile)
            .await?;
        user_id
    };

    //create access and refresh token for the user
    let session_id = Uuid::new_v4();
    let token_pair = state
        .auth_service
        .jwt_service
        .create_token_pair(&user_id, &session_id)?;

    state
        .db_repo
        .as_ref()
        .refresh_token_repo
        .create_refresh_token(&token_pair.refresh_token)
        .await?;

    Ok(token_pair)
}


async fn rotate_session(
    state: &UnauthenticatedSharedState,
    refresh_token: &str,
) -> Result<TokenPair> {
    let jwt_service = JwtService::new()?;
    let decoded_refresh_token = jwt_service.decode_refresh_token(&refresh_token)?;

    //rotating token
    let token_pair = state
        .auth_service
        .jwt_service
        .rotate_token(&decoded_refresh_token)?;

    //updating refresh token as it will be rotated
    state
        .db_repo
        .refresh_token_repo
        .create_refresh_token(&decoded_refresh_token)
        .await?;

    //create entry for new refresh token
    state
        .db_repo
        .as_ref()
        .refresh_token_repo
        .create_refresh_token(&token_pair.refresh_token)
        .await?;

    Ok(token_pair)
}


