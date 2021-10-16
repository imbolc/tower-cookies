//! [`Future`] types.

use crate::Cookies;
use futures_util::ready;
use http::{header, HeaderValue, Response};
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    /// Response future for [`CookieManager`].
    #[derive(Debug)]
    pub struct ResponseFuture<F> {
        #[pin]
        pub(crate) future: F,
        pub(crate) cookies: Cookies,
    }
}

impl<F, ResBody, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut res = ready!(this.future.poll(cx)?);

        let mut cookies = this.cookies.inner.lock();
        if cookies.changed {
            let values: Vec<_> = cookies
                .jar()
                .delta()
                .filter_map(|c| HeaderValue::from_str(&c.to_string()).ok())
                .collect();
            let headers = res.headers_mut();
            for value in values {
                headers.append(header::SET_COOKIE, value);
            }
        }

        Poll::Ready(Ok(res))
    }
}
