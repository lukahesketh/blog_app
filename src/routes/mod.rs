use axum::{Router, routing::get, routing::post};
mod create_account;
mod create_post;
mod get_posts;
mod login;
use crate::DatabaseConnectStruct;
use create_account::create_account;
use create_post::create_post;
use get_posts::get_posts;
use login::login;
use tower_http::cors::CorsLayer;

pub fn axum_router(state: DatabaseConnectStruct) -> Router {
    Router::new()
        .route("/create_account", post(create_account))
        .route("/login", post(login))
        .route("/get_posts", get(get_posts))
        .route("/create_post", post(create_post))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
