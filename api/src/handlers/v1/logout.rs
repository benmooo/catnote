use actix_web::{cookie::Cookie, post, HttpResponse};
use serde_json::json;

use crate::{
    err::AppErr,
    handlers::{REFRESH_TOKEN_COOKIE_DOMAIN, REFRESH_TOKEN_COOKIE_PATH}, mw::Auth,
};

// Need login status to logout 
#[post("/logout", wrap="Auth")]
pub async fn logout() -> Result<HttpResponse, AppErr> {
    // we can't revoke the access token instanly, but we can revoke the refresh_token
    let mut cookie = Cookie::build("refresh_token", "")
        .domain(REFRESH_TOKEN_COOKIE_DOMAIN)
        .path(REFRESH_TOKEN_COOKIE_PATH)
        .secure(true)
        .http_only(true)
        .finish();
    cookie.make_removal();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        // .cookie(cookie)
        .json(json!({"data": "success"})))
}
