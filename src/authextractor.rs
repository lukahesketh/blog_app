use crate::DatabaseConnectStruct;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthUser(pub Claims);

impl FromRequestParts<DatabaseConnectStruct> for AuthUser {
    type Rejection = StatusCode;

    fn from_request_parts(
        parts: &mut Parts,
        state: &DatabaseConnectStruct,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send>> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let jwt_key = state.jwt_private_key.clone();

        Box::pin(async move {
            let auth_header = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or(StatusCode::UNAUTHORIZED)?;

            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_key.as_bytes()),
                &Validation::default(),
            )
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

            Ok(AuthUser(data.claims))
        })
    }
}
