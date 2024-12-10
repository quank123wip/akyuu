use axum::{
    routing::get,
    Router,
};

use crate::AppState;
use crate::handler;

pub fn setup_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler::greet))
        .route("/:id", get(handler::get_post_by_id).post(handler::post_post_by_id).put(handler::post_post_by_id).delete(handler::delete_post_by_id))
        .with_state(state)
}


