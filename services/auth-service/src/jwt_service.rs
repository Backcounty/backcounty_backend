use chrono::{DateTime, Duration};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Deserializer, Serialize};
use crate::error::Result;

//This are the claims that the jwt requires to sign it
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iss: String,
    pub iat: DateTime<Utc>,
    pub exp: u64,
    pub sub: uuid::Uuid,
    pub aud: String,
    pub jti: uuid::Uuid,
    pub session_id:uuid::Uuid,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    encoding_secret: String,
    decoding_key: DecodingKey,
    decoding_secret: String,
    header: Header,
    algorithm: Algorithm,
}

#[derive(Debug, Serialize)]
pub struct AccessToken(pub String, pub Claims);

#[derive(Debug, Serialize)]
pub struct RefreshToken(pub String, pub Claims);

#[derive(Debug)]
pub struct TokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl JwtService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            encoding_key: EncodingKey::from_secret(dotenv::var("ENCODING_KEY")?.as_bytes()),
            encoding_secret: dotenv::var("ENCODING_KEY")?,
            decoding_key: DecodingKey::from_secret(dotenv::var("ENCODING_KEY")?.as_bytes()),
            decoding_secret: dotenv::var("DECODING_KEY")?,
            header: Header::new(Algorithm::HS256),
            algorithm: Algorithm::HS256,
        })
    }

    pub fn create_token_pair(&self, user_id: &uuid::Uuid,session_id:&uuid::Uuid) -> Result<TokenPair> {
        let now = Utc::now().timestamp() as u64;
        let exp = now.saturating_add(Duration::minutes(15).num_seconds() as u64);

        let access_token_claims = Claims {
            iat: Utc::now(),
            iss: "https://auth_service.backcountry.com".to_string(),
            aud: "https://api.backcountry.com".to_string(),
            sub: user_id.clone(),
            exp,
            jti: uuid::Uuid::new_v4(),
            session_id:session_id.clone()
        };

        let refresh_token_expiration = Duration::days(10).num_seconds() as u64;
        let refresh_token_claims = Claims {
            exp: refresh_token_expiration,
            jti: uuid::Uuid::new_v4(),
            ..access_token_claims.clone()
        };

        let access_token = encode(&self.header, &access_token_claims, &self.encoding_key)?;
        let refresh_token = encode(&self.header, &refresh_token_claims, &self.encoding_key)?;

        Ok({
            TokenPair {
                access_token: AccessToken(access_token, access_token_claims),
                refresh_token: RefreshToken(refresh_token, refresh_token_claims),
            }
        })
    }

    pub fn decode_refresh_token(&self, refresh_token: &str) -> Result<RefreshToken> {
        let claims = decode::<Claims>(refresh_token, &self.decoding_key, &Validation::new(self.algorithm))?.claims;
        let refresh_token_struct = RefreshToken(refresh_token.to_owned(), claims);
        Ok(refresh_token_struct)
    }

    pub fn rotate_token(&self, current_token: &RefreshToken) -> Result<TokenPair> {
        let now = Utc::now().timestamp() as u64;
        let exp = now.saturating_add(Duration::minutes(15).num_seconds() as u64);
        let iat=Utc::now();
        
        let access_token_claims = Claims {
            iat,
            jti:uuid::Uuid::new_v4(),
            exp,
            ..current_token.1.clone()
        };
        
        let refresh_token_claims=Claims{
            iat,
            jti:uuid::Uuid::new_v4(),
            ..current_token.1.clone()
        };
        let access_token = encode(&self.header, &access_token_claims, &self.encoding_key)?;
        let refresh_token = encode(&self.header, &refresh_token_claims, &self.encoding_key)?;
     
        Ok({
            TokenPair {
                access_token: AccessToken(access_token, access_token_claims),
                refresh_token: RefreshToken(refresh_token, refresh_token_claims),
            }
        })
    }
}
