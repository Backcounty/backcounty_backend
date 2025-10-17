use model::UserProfile;

use crate::constants;
use super::types::{GoogleAccessTokenRequest, GoogleAccessTokenResponse, GoogleProfileDto};
use crate::error::Result;

pub(super) struct GoogleOauthService(reqwest::Client);

impl GoogleOauthService {
    pub(super) fn new() -> Self {
        Self(reqwest::Client::new())
    }

    pub(super) async fn get_oauth_access_token(
        &self,
        authorization_code: &str,
    ) -> Result<GoogleAccessTokenResponse> {
        let token_request = GoogleAccessTokenRequest {
            code: authorization_code.to_string(),
            client_id: dotenv::var("CLIENT_ID")?,
            client_secret: dotenv::var("CLIENT_SECRET")?,
            redirect_uri: dotenv::var("REDIRECT_URI")?,
            grant_type: dotenv::var("GRANT_TYPE")?,
        };
        let response = self
            .0
            .post(constants::GOOGLE_OAUTH_TOKEN_ENDPOINT)
            .form(&token_request)
            .send()
            .await?;
        let token_response = response.json::<GoogleAccessTokenResponse>().await?;

        Ok(token_response)
    }

    pub(super) async fn get_user_profile(&self, token: &str) -> Result<UserProfile> {
        let person_fields = constants::PERSON_FIELDS.join(",");
        //Exchange token to get google profile
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