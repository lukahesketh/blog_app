use crate::{DatabaseConnectStruct, authextractor::AuthUser};
use axum::{Json, extract::Path, extract::State, http::HeaderMap};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use sqlx::query;

pub async fn like_post(
    AuthUser(claims): AuthUser,
    Path(post_id_2_like): Path<u32>,
    State(state): State<DatabaseConnectStruct>,
) {
}
