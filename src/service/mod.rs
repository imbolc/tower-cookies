//! Middleware to use [`Cookies`].

use self::future::ResponseFuture;
use crate::Cookies;
use http::{header, Request, Response};
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;

pub mod future;

/// Middleware to use [`Cookies`].
#[derive(Clone, Debug)]
pub struct CookieManager<S> {
    inner: S,
}

impl<S> CookieManager<S> {
    /// Create a new cookie manager.
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<ReqBody, ResBody, S> Service<Request<ReqBody>> for CookieManager<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let value = req
            .headers()
            .get_all(header::COOKIE)
            .iter()
            .cloned()
            .collect();
        let cookies = Cookies::new(value);
        req.extensions_mut().insert(cookies.clone());

        ResponseFuture {
            future: self.inner.call(req),
            cookies,
        }
    }
}

/// Layer to apply [`CookieManager`] middleware.
#[derive(Clone, Debug, Default)]
pub struct CookieManagerLayer {
    _priv: (),
}

impl CookieManagerLayer {
    /// Create a new cookie manager layer.
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl<S> Layer<S> for CookieManagerLayer {
    type Service = CookieManager<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CookieManager { inner }
    }
}
