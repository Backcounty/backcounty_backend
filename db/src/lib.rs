mod repo;
mod error;

use error::*;
use sea_orm::Database;

use crate::Result;

async fn init()->Result<()>{
    let connection=Database::connect(dotenv::var("DATABASE_URL")?).await?;
    Ok(())
}
