use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
}

pub struct BearerToken(pub String);