use crate::Cookies;
use async_trait::async_trait;
use axum_core::extract::{FromRequest, RequestParts};
use http::StatusCode;

#[async_trait]
impl<B> FromRequest<B> for Cookies
where
    B: Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let extensions = req.extensions().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract cookies: extensions has been taken by another extractor",
        ))?;
        extensions.get::<Cookies>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract cookies. Is `CookieManagerLayer` enabled?",
        ))
    }
}
