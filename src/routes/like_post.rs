use crate::{DatabaseConnectStruct, authextractor::AuthUser};
use axum::{Json, extract::Path, extract::State, http::HeaderMap};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use sqlx::query;
#[derive(Deserialize)]
pub struct Payload {
    pub target_post_id: i32,
}
#[derive(Serialize, sqlx::FromRow)]
pub struct LikeResponse {
    pub post_id: i32,
    pub like_count: i64,
}
pub async fn like_post(
    AuthUser(claims): AuthUser,
    State(state): State<DatabaseConnectStruct>,
    Json(payload): Json<Payload>,
) -> Json<LikeResponse> {
    let user_id = claims.sub;
    let target_id = payload.target_post_id;

    let sql_query: LikeResponse = sqlx::query_as(
        "INSERT INTO likes (user_id, post_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        RETURNING (SELECT COUNT(*) FROM likes WHERE post_id = $2);",
    )
    .bind(&user_id)
    .bind(target_id)
    .fetch_one(&state.db)
    .await
    .unwrap();

    Json(LikeResponse {
        post_id: target_id,
        like_count: sql_query.like_count,
    })
}
