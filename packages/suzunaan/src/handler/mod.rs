use axum::{
    extract::Path,
    extract::State,
    Json,
};
use sea_orm::*;
//use serde_json::Value;

use crate::error::AppError;
use crate::AppState;
use crate::entities::{post, prelude::*};


pub async fn greet(State(state): State<AppState>) -> String {
    state.config.greet_message.clone()
}

pub async fn get_post_by_id(State(state): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    let post = Post::find().filter(post::Column::Id.eq(id)).one(&state.db).await;
    match post {
        Ok(Some(p)) => Ok(Json(serde_json::json!(p))),
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(AppError::DatabaseError(e.to_string())),
    }
}
