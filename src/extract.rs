use crate::Cookies;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use http::StatusCode;

#[async_trait]
impl<B> FromRequest<B> for Cookies
where
    B: Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let extensions = match req.extensions() {
            Some(exts) => exts,
            None => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Can't extract cookies: extensions has been taken by another extractor",
                ))
            }
        };
        match extensions.get::<Cookies>() {
            Some(cookies) => Ok(cookies.clone()),
            None => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract cookies. Is CookieLayer enabled?",
            )),
        }
    }
}
