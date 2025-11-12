use crate::middleware::Ctx;
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use crate::Result;
#[derive(Deserialize)]
pub struct Blog{
    pub title: String,
    pub content: String,
}

pub async fn create_blog(ctx:Extension<Ctx>,blog:Json<Blog>)->Result<Response>{
    println!("Title {:?}",blog.title);
    println!("Content {:?}",blog.content);
    Ok("".into_response())
}