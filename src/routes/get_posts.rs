use crate::DatabaseConnectStruct;
use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FeedQuery {
    last_id: Option<i32>,
}

#[derive(Serialize)]
struct Post {
    id: i32,
    username: String,
    content: String,
    background_url: Option<String>,
    audio_url: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_posts(
    State(state): State<DatabaseConnectStruct>,
    Query(params): Query<FeedQuery>,
) -> Json<serde_json::Value> {
    let posts = match params.last_id {
        None => sqlx::query_as!(
            Post,
            "SELECT posts.id, users.username, posts.content, posts.background_url, posts.audio_url, posts.created_at
             FROM posts
             JOIN users ON posts.user_id = users.id
             ORDER BY posts.created_at DESC
             LIMIT 1"
        )
        .fetch_all(&state.db)
        .await
        .unwrap(),

        Some(last_id) => sqlx::query_as!(
            Post,
            "SELECT posts.id, users.username, posts.content, posts.background_url, posts.audio_url, posts.created_at
             FROM posts
             JOIN users ON posts.user_id = users.id
             WHERE posts.id < $1
             ORDER BY posts.created_at DESC
             LIMIT 1",
            last_id
        )
        .fetch_all(&state.db)
        .await
        .unwrap(),
    };

    Json(serde_json::json!({ "posts": posts }))
}
