use crate::CookieService;
use tower_layer::Layer;

#[derive(Clone, Debug)]
pub struct CookieLayer;

impl<S> Layer<S> for CookieLayer {
    type Service = CookieService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CookieService { inner }
    }
}
