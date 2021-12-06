use hyper::{Body, Response};
mod index;
mod login;

pub use index::*;
pub use login::*;

pub fn handle_admin_404() -> Response<Body> {
    Response::new(Body::from("此路不通"))
}
