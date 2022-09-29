use actix_web::{post, web, HttpResponse};
use bson::DateTime;
use chrono::{Duration, Utc};
use serde_json::json;
use validator::Validate;

use crate::{
    context::Pool,
    err::AppErr,
    models::{NewUser, VCode, VCodeType},
    services::{rand_str, EmailService},
};

#[post("/signup")]
pub async fn signup(
    pool: web::Data<Pool>,
    user: web::Json<NewUser>,
    email_service: web::Data<EmailService>,
) -> Result<HttpResponse, AppErr> {
    user.validate()?;
    let new_user = user.0.to_hashed_password()?;

    // write to db -> mark email un-verified
    let _insert_id = pool.user_repo().new_user(&new_user).await?;

    // generate verification code and send to user email
    let code = rand_str(VERIFICATION_CODE_LEN as usize).to_uppercase();
    email_service.send_vcode(&new_user.email, &code)?;

    // writes to vcode collection
    let exp_at = exp_from_now(Duration::minutes(15));
    let vcode = VCode::new(new_user.email, code, VCodeType::VerifyEmail, exp_at);

    pool.vcode_repo().new_vcode(vcode).await?;
    Ok(HttpResponse::Ok().json(json!({"data": "verification code has been sent to your email!"})))
}

pub static VERIFICATION_CODE_LEN: u8 = 6;

pub fn exp_from_now(dur: Duration) -> DateTime {
    DateTime::from_chrono(Utc::now() + dur)
}
