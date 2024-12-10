use axum::http::StatusCode;
use axum::{extract::Json, extract::Path, extract::State};
use sea_orm::*;
use serde_json::json;

use crate::entities::sea_orm_active_enums::Permission;
use crate::entities::{post, prelude::*};
use crate::error::AppError;
use crate::AppState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPost {
    #[serde(rename = "secret", default)]
    pub secret: Option<String>,
    #[serde(rename = "password", default)]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewPost {
    #[serde(rename = "title", default)]
    pub title: Option<String>,
    #[serde(rename = "text", default)]
    pub text: Option<String>,
    #[serde(rename = "author", default)]
    pub author: Option<String>,
    #[serde(rename = "author_email", default)]
    pub author_email: Option<String>,
    #[serde(rename = "secret", default)]
    pub secret: Option<String>,
    #[serde(rename = "password", default)]
    pub password: Option<String>,
    #[serde(rename = "permission")]
    pub permission: Option<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePost {
    pub secret: Option<String>,
}

pub async fn greet(State(state): State<AppState>) -> String {
    state.config.greet_message.clone()
}

pub async fn get_post_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<GetPost>,
) -> Result<Json<serde_json::Value>, AppError> {
    let post = Post::find()
        .filter(post::Column::Id.eq(id))
        .one(&state.db)
        .await;
    match post {
        Ok(Some(p)) => match p.permission {
            Permission::Public => Ok(Json(serde_json::json!(p))),
            Permission::Private => {
                if payload
                    .secret
                    .is_some_and(|x| x == *p.secret.as_ref().unwrap_or(&String::new()))
                {
                    Ok(Json(serde_json::json!(p)))
                } else if payload
                    .password
                    .is_some_and(|x| x == *p.password.as_ref().unwrap_or(&String::new()))
                {
                    Ok(Json(serde_json::json!(p)))
                } else {
                    Err(AppError::Unauthorized)
                }
            }
            Permission::Restricted => {
                if payload
                    .secret
                    .is_some_and(|x| x == *p.secret.as_ref().unwrap_or(&String::new()))
                {
                    Ok(Json(serde_json::json!(p)))
                } else {
                    Err(AppError::Unauthorized)
                }
            }
        },
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(AppError::DatabaseError(e.to_string())),
    }
}

pub async fn post_post_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<NewPost>,
) -> Result<StatusCode, AppError> {
    let query = Post::find()
        .filter(post::Column::Id.eq(id.clone()))
        .one(&state.db)
        .await;
    
    if payload.title.is_none() && 
       payload.text.is_none() && 
       payload.author.is_none() && 
       payload.author_email.is_none() && 
       payload.secret.is_none() && 
       payload.password.is_none() &&
       payload.permission.is_none() {
        return Err(AppError::BadRequest("Payload is empty".to_string()));
    }
    
    match query {
        Ok(Some(p)) => match p.permission {
            Permission::Public => {
                let mut post = p.into_active_model();
                post.set_from_json(json!(payload)).unwrap_err();
                match post.update(&state.db).await {
                    Ok(_) => Ok(StatusCode::OK),
                    Err(e) => Err(AppError::DatabaseError(e.to_string())),
                }
            }
            _ => {
                if payload.clone().secret.is_some_and(|x| x == *p.secret.as_ref().unwrap_or(&String::new())) {
                    let mut post = p.into_active_model();
                    post.set_from_json(json!(payload)).unwrap_err();
                    match post.update(&state.db).await {
                        Ok(_) => Ok(StatusCode::OK),
                        Err(e) => Err(AppError::DatabaseError(e.to_string())),
                    }
                } else {
                    Err(AppError::Unauthorized)
                }
            }
        },
        Ok(None) => {
            let mut post = post::ActiveModel {
                index: ActiveValue::NotSet,
                id: ActiveValue::Set(id),
                title: ActiveValue::NotSet,
                text: ActiveValue::NotSet,
                created_at: ActiveValue::NotSet,
                updated_at: ActiveValue::NotSet,
                author: ActiveValue::NotSet,
                author_email: ActiveValue::NotSet,
                permission: ActiveValue::NotSet,
                secret: ActiveValue::NotSet,
                password: ActiveValue::NotSet,
            };
            post.set_from_json(json!(payload)).unwrap_err();
            match post.insert(&state.db).await {
                Ok(_) => Ok(StatusCode::CREATED),
                Err(e) => Err(AppError::DatabaseError(e.to_string())),
            }
        }
        Err(e) => Err(AppError::DatabaseError(e.to_string())),
    }
}

pub async fn delete_post_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<DeletePost>,
) -> Result<StatusCode, AppError> {
    let post = Post::find()
        .filter(post::Column::Id.eq(id))
        .one(&state.db)
        .await;

    match post {
        Ok(res) => {
            if res.is_some() {
                let res = res.unwrap();
                let id = res.id;
                let perm = res.permission;
                let secret = res.secret;
                match perm {
                    Permission::Public => match Post::delete_many()
                        .filter(post::Column::Id.eq(id))
                        .exec(&state.db)
                        .await
                    {
                        Ok(_) => Ok(StatusCode::NO_CONTENT),
                        Err(e) => Err(AppError::DatabaseError(e.to_string())),
                    },
                    _ => {
                        if payload.secret.is_some_and(|x| x == secret.unwrap_or(String::new())) {
                            match Post::delete_many()
                                .filter(post::Column::Id.eq(id))
                                .exec(&state.db)
                                .await
                            {
                                Ok(_) => Ok(StatusCode::NO_CONTENT),
                                Err(e) => Err(AppError::DatabaseError(e.to_string())),
                            }
                        } else {
                            Err(AppError::Unauthorized)
                        }
                    }
                }
            } else {
                Err(AppError::NotFound)
            }
        }
        Err(e) => Err(AppError::DatabaseError(e.to_string())),
    }
}
