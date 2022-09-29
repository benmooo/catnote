use actix_web::{cookie::Cookie, post, web, HttpRequest, HttpResponse};
use chrono::Duration;
use serde_json::json;

use crate::{err::AppErr, services::JwtService};

#[post("/refresh-token")]
pub async fn refresh_token(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, AppErr> {
    let rt = req.cookie("refresh_token").ok_or(AppErr::Unauthorized)?;
    let claims = jwt_service.verify(rt.value())?;

    // we should read user info from db, because userinfo might be updated
    // however, the access token only last for 15mins, so client will hit this endpoint 
    // more frequently, as a result we skip this process, and the user infomation should be 
    // cached by client [localstorage] during login. 
    // let user = ctx.user_repo().user_by_id(&claims.id).await?;

    let at_claim = claims.clone().exp_from_now(Duration::minutes(30));
    let rt_claim = claims.exp_from_now(Duration::days(7));

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
        .json(json!({"data": {"access_token": at}})))
}

pub static REFRESH_TOKEN_COOKIE_PATH: &str = "/api/v1/rest/refresh-token";
pub static REFRESH_TOKEN_COOKIE_DOMAIN: &str = "localhost";