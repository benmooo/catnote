use bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationCode {
    // #[serde(rename(deserialize = "_id"))]
    // pub id: ObjectId,
    pub email: String,
    pub code: String,
    pub used_for: VerificationCodeType,
    pub created_at: DateTime,
    pub expire_at: DateTime,
}

impl VerificationCode {
    pub fn new(
        email: String,
        code: String,
        used_for: VerificationCodeType,
        expire_at: DateTime,
    ) -> Self {
        Self {
            email,
            code,
            used_for,
            expire_at,
            created_at: DateTime::now(),
        }
    }
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VerificationCodeType {
    VerifyEmail = 0,
    ResetPassword = 1,
}

pub type VCode = VerificationCode;
pub type VCodeType = VerificationCodeType;
