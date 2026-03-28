use crate::DatabaseConnectStruct;
use axum::{Json, extract::State, http::HeaderMap};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use sqlx::query;

#[derive(Deserialize)]
pub struct PostBody {
    content: String,
    background_url: Option<String>,
    audio_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub async fn create_post(
    State(state): State<DatabaseConnectStruct>,
    headers: HeaderMap,
    Json(body): Json<PostBody>,
) -> Json<serde_json::Value> {
    let auth_header = headers.get("Authorization").unwrap();
    let token = &auth_header.to_str().unwrap()[7..];
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_private_key.as_ref()),
        &Validation::default(),
    )
    .unwrap()
    .claims;

    let user = sqlx::query!("SELECT id FROM users WHERE username = $1", claims.sub)
        .fetch_one(&state.db)
        .await
        .unwrap();

    query(
        "INSERT INTO posts (user_id, content, background_url, audio_url) VALUES ($1, $2, $3, $4)",
    )
    .bind(user.id)
    .bind(&body.content)
    .bind(&body.background_url)
    .bind(&body.audio_url)
    .execute(&state.db)
    .await
    .unwrap();

    Json(serde_json::json!({ "success": true }))
}
