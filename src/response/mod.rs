use hyper::{Body, Response, StatusCode};

pub trait Respond {
    fn respond(self) -> Response<Body>;
}

impl Respond for &'static str {
    fn respond(self) -> Response<Body> {
        Response::new(Body::from(self))
    }
}

impl Respond for (StatusCode, &'static str) {
    fn respond(self) -> Response<Body> {
        let mut resp = Response::new(Body::from(self.1));
        *resp.status_mut() = self.0;
        resp
    }
}
