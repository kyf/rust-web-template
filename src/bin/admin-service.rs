#[macro_use]
extern crate log;
use hospital::*;
use hyper::{
    body::to_bytes,
    header,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Server,
};
use std::{convert::Infallible, env};

const ADDR: &'static str = "0.0.0.0:6868";

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let app = make_service_fn(|conn: &AddrStream| {
        let addr = conn.remote_addr();

        async move {
            Ok::<_, Infallible>(service_fn(move |mut req: Request<Body>| async move {
                let method = req.method().clone();
                let uri = req.uri().clone();
                let headers = req.headers().clone();
                let user_agent = headers
                    .get(header::USER_AGENT)
                    .and_then(|it| Some(it.to_str().unwrap_or("")))
                    .unwrap_or("unknown");
                let token = headers.get("token");

                //middleware
                if let Some(r) = auth(&uri, token).await {
                    return Ok(r.respond());
                }

                let body = to_bytes(req.body_mut()).await.unwrap();
                let mut resp = match (method.clone(), uri.path()) {
                    (Method::GET, "/") | (Method::GET, "/index") => {
                        handle_admin_index().await.respond()
                    }
                    //登录
                    (Method::POST, "/admin/login") => handle_admin_login(&body).await.respond(),
                    _ => handle_admin_404(),
                };

                info!(
                    "{}   {}   {}   {}   {}",
                    addr,
                    method,
                    uri.path(),
                    resp.status(),
                    user_agent
                );

                let content_type = resp.headers().get(header::CONTENT_TYPE);
                if content_type.is_none() {
                    resp.headers_mut().insert(
                        header::CONTENT_TYPE,
                        "text/json;charset=utf-8".parse().unwrap(),
                    );
                }

                Ok::<_, Infallible>(resp)
            }))
        }
    });

    let server = Server::bind(&ADDR.parse().unwrap());
    info!("listen [{}] ...", ADDR);
    server.serve(app).await.unwrap();
}
