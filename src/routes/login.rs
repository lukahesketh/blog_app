use crate::DatabaseConnectStruct;
use axum::{Json, extract::State};
use bcrypt::verify;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::query_as;

#[derive(Deserialize)]
pub struct AccountLogin {
    username: String,
    password: String,
}

struct User {
    id: i32,
    username: String,
    password_hash: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(
    State(state): State<DatabaseConnectStruct>,
    Json(account_struct): Json<AccountLogin>,
) -> Json<serde_json::Value> {
    let user = query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        account_struct.username
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    let is_valid = verify(&account_struct.password, &user.password_hash).unwrap();

    if !is_valid {
        return Json(serde_json::json!({ "error": "invalid credentials" }));
    }

    let claims = Claims {
        sub: user.username,
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
