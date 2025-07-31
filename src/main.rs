mod error;

use std::sync::Arc;
use db::Db;
use routes::RouterService;

use crate::error::Result;

#[tokio::main]
async fn main()->Result<()> {
    let db=Db::init().await?;
    let router_service=RouterService::init(Arc::new(db)).await?;
    router_service.run().await?;

    Ok(())

}