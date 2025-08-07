mod types;

use axum::extract::State;
use axum::response::{IntoResponse, Response};

use model::UserProfile;

use self::types::{
    AuthorizationCode, GoogleAccessTokenRequest, GoogleAccessTokenResponse, GoogleProfileDto,
};
use super::super::UnauthenticatedSharedState;
use crate::constants;
use crate::Result;

pub async fn handle_oauth(
    State(state): State<UnauthenticatedSharedState>,
    authorization_code: String,
) -> Result<Response> {
    let authorization_code_struct =
        serde_json::from_str::<AuthorizationCode>(authorization_code.as_str())?;

    let google_oauth_client = GoogleOauthClient::new();
    let access_token = google_oauth_client
        .get_access_token(authorization_code_struct.code)
        .await?;
    let user_profile = google_oauth_client
        .get_user_profile(access_token.access_token)
        .await?;

    state
        .db_repo
        .as_ref()
        .user_repo
        .create_user(user_profile)
        .await?;

    Ok(().into_response())
}

struct GoogleOauthClient(reqwest::Client);

impl GoogleOauthClient {
    fn new() -> Self {
        Self(reqwest::Client::new())
    }

    async fn get_access_token(
        &self,
        authorization_code: String,
    ) -> Result<GoogleAccessTokenResponse> {
        let token_request = GoogleAccessTokenRequest {
            code: authorization_code,
            client_id: dotenv::var("CLIENT_ID")?,
            client_secret: dotenv::var("CLIENT_SECRET")?,
            redirect_uri: dotenv::var("REDIRECT_URI")?,
            grant_type: dotenv::var("GRANT_TYPE")?,
        };
        let response = self.0
            .post(constants::GOOGLE_OAUTH_TOKEN_ENDPOINT)
            .form(&token_request)
            .send()
            .await?;
        let token_response = response.json::<GoogleAccessTokenResponse>().await?;

        Ok(token_response)
    }

    async fn get_user_profile(&self, token: String) -> Result<UserProfile> {
        let person_fields = constants::PERSON_FIELDS.join(",");
        let google_profile_response = self
            .0
            .get(constants::GOOGLE_PROFILE_URL)
            .bearer_auth(&token)
            .query(&[("personFields", &person_fields)])
            .send()
            .await?;

        let google_profile_dto = google_profile_response.json::<GoogleProfileDto>().await?;
        let user_profile: UserProfile = google_profile_dto.into();

        Ok(user_profile)
    }
}
