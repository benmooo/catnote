use std::sync::Arc;

use bson::doc;
use log::info;
use mongodb::{Collection, Database};

use crate::{
    err::AppErr,
    models::{VerificationCode, VerificationCodeType},
};

pub struct VerificationCodeRepository {
    db: Arc<Database>,
}

impl VerificationCodeRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn vcode(
        &self,
        email: &str,
        code: &str,
        used_for: VerificationCodeType,
    ) -> Result<VerificationCode, AppErr> {
        let vcode_collection: Collection<VerificationCode> = self.db.collection("v-codes");

        info!("db_io: find vcode by email & type!");
        vcode_collection
            .find_one(
                doc! { "email": email, "code": code, "used_for": used_for as u32},
                None,
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .ok_or(AppErr::ResourceNotFound(
                "verification not found".to_owned(),
            ))
    }

    pub async fn find_one_and_del(
        &self,
        email: &str,
        code: &str,
        used_for: VerificationCodeType,
    ) -> Result<VerificationCode, AppErr> {
        let vcode_collection: Collection<VerificationCode> = self.db.collection("v-codes");

        info!("db_io: find vcode by email & type!");
        vcode_collection
            .find_one_and_delete(
                doc! { "email": email, "code": code, "usedFor": used_for as i32},
                None,
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .ok_or(AppErr::ResourceNotFound(
                "verification code not found".to_owned(),
            ))
    }

    // create index on { email -> unique, vcodename -> unique }
    pub async fn new_vcode(&self, vcode: VerificationCode) -> Result<(), AppErr> {
        let vcode_collection = self.db.collection("v-codes");

        info!("db_io: insert new vcode");
        vcode_collection
            .insert_one(vcode, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))
            .map(|_| ())
    }
}

pub type VCodeRepo = VerificationCodeRepository;