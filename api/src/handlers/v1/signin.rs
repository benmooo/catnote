use actix_web::{cookie::Cookie, post, web, HttpResponse};
use chrono::Duration;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    context::Pool,
    err::AppErr,
    services::{argon2_verify, Claims, JwtService}, handlers::{REFRESH_TOKEN_COOKIE_DOMAIN, REFRESH_TOKEN_COOKIE_PATH},
};

#[post("/signin")]
pub async fn signin(
    pool: web::Data<Pool>,
    payload: web::Json<SigninPayload>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, AppErr> {
    // validate
    payload
        .validate()
        .map_err(|_| AppErr::EmailPasswordNotMatch)?;

    let user = pool
        .user_repo()
        .user_by_email(&payload.email)
        .await
        .map_err(|err| match err {
            AppErr::ResourceNotFound(_) => AppErr::EmailPasswordNotMatch,
            _ => err,
        })?;

    // verify email
    if !argon2_verify(&payload.password, &user.password) {
        return Err(AppErr::EmailPasswordNotMatch);
    }

    // email verified?
    if !user.email_verified {
        return Err(AppErr::EmailNotVerified);
    }

    // return user & access_token  [ refresh_token in http cookie ]
    let at_claim = Claims::default()
        .id(&user.id.to_string())
        .name(&user.username)
        .exp_from_now(Duration::minutes(30));
    let rt_claim = Claims::default()
        .id(&user.id.to_string())
        .name(&user.username)
        .exp_from_now(Duration::days(7));

    let at = jwt_service.issue(&at_claim)?;
    let rt = jwt_service.issue(&rt_claim)?;
    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", rt)
                .domain(REFRESH_TOKEN_COOKIE_DOMAIN)
                .path(REFRESH_TOKEN_COOKIE_PATH)
                .secure(true)
                .http_only(true)
                .finish(),
        )
        .json(json!({"data": {"user": user, "access_token": at}})))
}

#[derive(Deserialize, Validate)]
pub struct SigninPayload {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 50))]
    password: String,
}
