use crate::DatabaseConnectStruct;
use axum::{Json, extract::State};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    id: i32,
    username: String,
    content: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_messages(State(state): State<DatabaseConnectStruct>) -> Json<serde_json::Value> {
    let messages = sqlx::query_as!(
        Message,
        "SELECT messages.id, users.username, messages.content, messages.created_at
         FROM messages
         JOIN users ON messages.user_id = users.id
         ORDER BY messages.created_at DESC
         LIMIT 50"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(serde_json::json!({ "messages": messages }))
}
