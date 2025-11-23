use crate::middleware::Ctx;
use crate::{AuthenticatedSharedState, Result, UnauthenticatedSharedState};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Blog {
    pub title: String,
    pub content: String,
}

pub async fn create_blog(
    state: State<AuthenticatedSharedState>,
    ctx: Extension<Ctx>,
    blog: Json<Blog>,
) -> Result<Response> {
    let title = blog.title.clone();
    let content = blog.content.clone();
    let author = ctx.user_info.user_id.clone();
    let blog_id = state
        .db_repo
        .blog_repo
        .create_blog(&author, &title, &content)
        .await?;
    let blog = state
        .db_repo
        .blog_repo
        .get_blog_by_blog_id(&blog_id)
        .await?;
    let blog_json=Json(serde_json::to_value(blog)?);

    Ok(blog_json.into_response())
}

#[derive(Deserialize)]
pub struct AuthorBlogQueryParam{
    is_published: bool,
}
pub async fn get_author_blogs_headers(
    state: State<AuthenticatedSharedState>,
    ctx: Extension<Ctx>,
    author_blog_query_param: Query<AuthorBlogQueryParam>,
) -> Result<Response> {
    let author_id = ctx.user_info.user_id;
    let is_published = author_blog_query_param.is_published;

    let blogs_headers = state
        .db_repo
        .blog_repo
        .list_author_blog_headers(&author_id, is_published)
        .await?;

    let blogs_headers_json=Json(blogs_headers);

    Ok(blogs_headers_json.into_response())
}
