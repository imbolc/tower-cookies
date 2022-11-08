use crate::Cookies;
use async_trait::async_trait;
use axum_core::extract::FromRequestParts;
use http::{request::Parts, StatusCode};

#[async_trait]
impl<S> FromRequestParts<S> for Cookies
where
    S: Sync + Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<Cookies>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract cookies. Is `CookieManagerLayer` enabled?",
        ))
    }
}
