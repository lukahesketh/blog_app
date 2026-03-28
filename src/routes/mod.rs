use axum::{Router, routing::get, routing::post};
mod create_account;
mod get_messages;
mod login;
use get_messages::get_messages;
mod send_message;
use crate::DatabaseConnectStruct;
use create_account::create_account;
use login::login;
use send_message::send_message;
use tower_http::cors::CorsLayer;

pub fn axum_router(state: DatabaseConnectStruct) -> Router {
    Router::new()
        .route("/create_account", post(create_account))
        .route("/login", post(login))
        .route("/get_messages", get(get_messages))
        .route("/send_message", post(send_message))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
