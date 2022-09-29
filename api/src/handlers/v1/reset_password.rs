use actix_web::{cookie::Cookie, post, web, HttpResponse};
use bson::doc;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    context::Pool,
    err::AppErr,
    handlers::{REFRESH_TOKEN_COOKIE_DOMAIN, REFRESH_TOKEN_COOKIE_PATH},
    models::VCodeType,
    services::argon2_hash,
};

#[post("/reset-password")]
pub async fn reset_password(
    pool: web::Data<Pool>,
    payload: web::Json<Payload>,
) -> Result<HttpResponse, AppErr> {
    // validate
    payload.validate()?;

    pool.vcode_repo()
        .find_one_and_del(&payload.email, &payload.code, VCodeType::ResetPassword)
        .await
        .map_err(|err| match err {
            AppErr::ResourceNotFound(_) => AppErr::IncorrectVerificationCode,
            _ => err,
        })?;

    // update password
    let update = doc! {"$set": {"password": argon2_hash(&payload.password)?}};
    pool.user_repo()
        .update_user_by_email(&payload.email, update)
        .await?;

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
        .json(json!({"data": "your password has been reset"})))
}

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(email)]
    pub email: String,
    #[validate(length(equal = 6))]
    pub code: String,
    #[validate(length(min = 8, max = 50))]
    password: String,
}
