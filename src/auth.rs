use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct BearerToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) if auth_header.starts_with("Bearer ") => {
                let token = auth_header.trim_start_matches("Bearer ").to_string();
                Ok(BearerToken(token))
            }
            _ => Err((
                StatusCode::UNAUTHORIZED,
                "The authorization token (Bearer Token) is missing",
            )),
        }
    }
}