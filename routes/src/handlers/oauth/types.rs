use serde::{Deserialize, Serialize};

use model::UserProfile;
#[derive(Debug, Deserialize)]
pub struct AuthorizationCode {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct GoogleAccessTokenRequest {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub grant_type: String,
}

#[derive(Deserialize)]
pub struct GoogleAccessTokenResponse {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GoogleProfileDto {
    pub names: Option<Vec<GoogleName>>,
    #[serde(rename = "emailAddresses")]
    pub email_addresses: Option<Vec<GoogleEmail>>,
    pub photos: Option<Vec<GooglePhoto>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GoogleName {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "givenName")]
    pub first_name: Option<String>,
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GoogleEmail {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GooglePhoto {
    pub url: String,
}



impl Into<UserProfile> for GoogleProfileDto {
    fn into(self) -> UserProfile {
        let (name, first_name, last_name) = self
            .names
            .unwrap_or_default()
            .get(0)
            .map(|x| {
                (
                    x.display_name.clone().unwrap_or_default(),
                    x.first_name.clone().unwrap_or_default(),
                    x.family_name.clone().unwrap_or_default(),
                )
            })
            .unwrap_or_default();

        let email = self
            .email_addresses
            .unwrap_or_default()
            .get(0)
            .map(|x| x.value.clone())
            .unwrap_or_default();

        let photo = self
            .photos
            .unwrap_or_default()
            .get(0)
            .map(|x| x.url.clone())
            .unwrap_or_default();

        UserProfile {
            name,
            first_name,
            last_name,
            email,
            photo,
        }
    }
}
