use crate::helper;
use auth_service::TokenPair;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum_cookie::CookieManager;
use serde::Deserialize;
use uuid::Uuid;

use super::super::UnauthenticatedSharedState;
use crate::{Error, Result};
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
    let token_pair = create_session(&state, &authorization_code_struct).await?;
    let token_response = helper::build_token_pair_response(token_pair)?;

    Ok(token_response.into_response())
}

async fn create_session(
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

pub async fn refresh_token(
    state: State<UnauthenticatedSharedState>,
    cookie_manager: CookieManager,
) -> Result<Response> {
    let sth=cookie_manager.cookie();
    println!("Cookie Jar:{:?}",sth);
    let refresh_token_cookie = cookie_manager
        .get("refresh_token")
        .ok_or(Error::EmptyRefreshToken)?
        .value()
        .to_string();

    let decoded_refresh_token = state
        .auth_service
        .jwt_service
        .decode_refresh_token(&refresh_token_cookie)?;

    //rotating token
    let token_pair = state
        .auth_service
        .jwt_service
        .rotate_token_from_refresh(&decoded_refresh_token)?;

    //updating refresh token as it will be rotated
    state
        .db_repo
        .refresh_token_repo
        .create_refresh_token(&decoded_refresh_token)
        .await?;

    //create new  entry for refresh token
    state
        .db_repo
        .as_ref()
        .refresh_token_repo
        .create_refresh_token(&token_pair.refresh_token)
        .await?;
    //Cookie header to store refresh token
    let token_response = helper::build_token_pair_response(token_pair)?;

    Ok(token_response.into_response())
}
