use hyper::{header::HeaderValue, StatusCode, Uri};

const WHITE_LIST: [&str; 3] = ["/login", "/", "/index"];

pub async fn auth(uri: &Uri, token: Option<&HeaderValue>) -> Option<(StatusCode, &'static str)> {
    for item in WHITE_LIST {
        if uri.path() == item {
            return None;
        }
    }

    let t = HeaderValue::from_static("123456");
    if Some(&t) == token {
        return None;
    }

    //info!("uri is {:?}, token is {:?}", uri, token);
    Some((StatusCode::from_u16(403).unwrap(), "deny access"))
}
