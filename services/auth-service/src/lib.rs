pub mod google_oauth_service;
pub mod jwt_service;
pub mod types;
mod constants;
mod error;
pub use error::*;
use google_oauth_service::*;
pub use jwt_service::*;
use model::UserProfile;

pub struct AuthService{
     google_oauth: GoogleOauthService,
    pub jwt_service: JwtService
}

impl AuthService {
    pub fn init() -> Result<AuthService> {
        Ok(AuthService{
            google_oauth:GoogleOauthService::new(),
            jwt_service:JwtService::new()?
        })
    }
    
    pub async fn get_profile_from_code(&self,authorization_code: &str) -> Result<UserProfile> {
        let token=self.google_oauth.get_oauth_access_token(authorization_code).await?;
        let user_profile=self.google_oauth.get_user_profile(&token.access_token).await?;
       
        Ok(user_profile)
    }
    
    
}

