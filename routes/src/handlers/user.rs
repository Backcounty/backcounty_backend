use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;
use serde_json::json;

use crate::error::Result;
use crate::middleware::Ctx;
use crate::{AuthenticatedSharedState};

pub async fn get_user_profile(
    ctx: Extension<Ctx>,
    state: State<AuthenticatedSharedState>,
) -> Result<Response> {
    let user_id = ctx.user_info.user_id.clone();
    let user_profile = state
        .db_repo
        .user_repo
        .get_user_by_user_id(&user_id)
        .await?;
    let user_profile_json=Json(json!({
        "name":user_profile.name,
        "email":user_profile.email,
        "profile":user_profile.profile_photo,
    }));

    Ok(user_profile_json.into_response())
}
