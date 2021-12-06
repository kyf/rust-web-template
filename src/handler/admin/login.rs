use crate::request::Extract;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
struct Account {
    username: String,
    password: String,
}

pub async fn handle_admin_login(body: &[u8]) -> (StatusCode, &'static str) {
    let a = Account::extract(body);
    info!("{:?}", a);
    (StatusCode::OK, "haha")
}
