use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
}

pub struct BearerToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{

}