use actix_web::{post, web, HttpResponse};
use chrono::{Duration};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    context::Pool,
    err::AppErr,
    models::{VCode, VCodeType},
    services::{rand_str, EmailService},
};

use super::{exp_from_now, VERIFICATION_CODE_LEN};

#[post("/send-vcode")]
pub async fn send_vcode(
    pool: web::Data<Pool>,
    email_service: web::Data<EmailService>,
    payload: web::Json<Payload>,
) -> Result<HttpResponse, AppErr> {
    payload.validate()?;

    let code = rand_str(VERIFICATION_CODE_LEN as usize).to_uppercase();
    email_service.send_vcode(&payload.email, &code)?;

    // writes to vcode collection
    let exp_at = exp_from_now(Duration::minutes(15));
    let vcode = VCode::new(payload.email.clone(), code, payload.code_type, exp_at);

    pool.vcode_repo().new_vcode(vcode).await?;

    Ok(HttpResponse::Ok().json(json!({"data": "verification code has been sent to your email!"})))
}

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(email)]
    pub email: String,
    #[serde(rename = "codeType")]
    pub code_type: VCodeType,
}
