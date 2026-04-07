use crate::DatabaseConnectStruct;
use axum::{Json, extract::State};
use bcrypt::{DEFAULT_COST, hash};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::query;

#[derive(Deserialize)]
pub struct AccountCreation {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn create_account(
    State(state): State<DatabaseConnectStruct>,
    Json(account): Json<AccountCreation>,
) -> Json<serde_json::Value> {
    let hashed_password = hash(&account.password, DEFAULT_COST).unwrap();

    let user = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
        &account.username,
        &hashed_password
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    let claims = Claims {
        sub: user.id.to_string(),
        exp: 9999999999,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_private_key.as_ref()),
    )
    .unwrap();

    Json(serde_json::json!({ "token": token }))
}
