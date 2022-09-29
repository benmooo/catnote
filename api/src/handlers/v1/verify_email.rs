use actix_web::{post, web, HttpResponse};
use bson::{doc, DateTime};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{context::Pool, err::AppErr, models::VerificationCodeType};



#[post("/verify-email")]
pub async fn verify_email(
    pool: web::Data<Pool>,
    payload: web::Json<VerifyEmailPayload>,
) -> Result<HttpResponse, AppErr> {
    // validate
    payload
        .validate()
        .map_err(|_| AppErr::IncorrectVerificationCode)?;

    pool.vcode_repo()
        .find_one_and_del(
            &payload.email,
            &payload.code,
            VerificationCodeType::VerifyEmail,
        )
        .await
        .map_err(|err| match err {
            AppErr::ResourceNotFound(_) => AppErr::IncorrectVerificationCode,
            _ => err,
        })?;

    // update user
    let _user = pool
        .user_repo()
        .update_user_by_email(
            &payload.email,
            doc! { "$set": {"emailVerified": true, "updatedAt": DateTime::now()}},
        )
        .await?;
    Ok(HttpResponse::Ok().json(json!({"data": "your email has been verified"})))
}

#[derive(Deserialize, Validate)]
pub struct VerifyEmailPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(equal = 6))]
    pub code: String,
}