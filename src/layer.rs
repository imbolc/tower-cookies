use crate::CookieService;
use tower_layer::Layer;

/// A layer to apply `CookieService` middleware
#[derive(Clone, Debug)]
pub struct CookieLayer;

impl<S> Layer<S> for CookieLayer {
    type Service = CookieService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CookieService { inner }
    }
}
