mod error;

use std::sync::Arc;
use db::Db;
use routes::RouterService;
use auth_service::AuthService;

use crate::error::Result;

#[tokio::main]
async fn main()->Result<()> {
    let db=Arc::new(Db::init().await?);
    let auth_service=Arc::new(AuthService::init()?);
    let router_service=RouterService::init(db,auth_service).await?;
    router_service.run().await?;

    Ok(())

}