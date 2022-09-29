use bson::{
    doc,
    oid::ObjectId,
    serde_helpers::{serialize_bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string},
    DateTime,
};
use juniper::graphql_object;
use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;

use super::{datetime_now, Note};
use crate::{
    err::{AppErr, AppResult},
    services::argon2_hash,
    Context,
};

// This user model is not for writing to database, but for write to http response
// In other world we should read from db, and deserialize it to this model, and serialize to http
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "serialize_object_id_as_hex_string"
    )]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email_verified: bool,

    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(default, serialize_with = "serialize_option_datetime")]
    pub updated_at: Option<DateTime>,
    #[serde(default, skip_serializing)]
    pub deleted_at: Option<DateTime>,
}

#[graphql_object(Context = Context)]
#[graphql(name="user")]
impl User {
    fn id(&self) -> String {
        self.id.to_hex()
    }

    fn username(&self) -> &str {
        self.username.as_str()
    }

    fn email(&self) -> &str {
        self.email.as_str()
    }

    fn email_verified(&self) -> bool {
        self.email_verified
    }

    fn created_at(&self) -> String {
        self.created_at.to_rfc3339_string()
    }

    fn updated_at(&self) -> Option<String> {
        self.updated_at.and_then(|t| Some(t.to_rfc3339_string()))
    }

    async fn notes(&self, ctx: &Context) -> AppResult<Vec<Note>> {
        ctx.note_repo().notes_by_author_id(&self.id).await
    }
}

// this model is used for deserialize from user input -> validate and then convert it into docs and writes to db
#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    #[validate(length(min = 6, max = 30))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[serde(skip_deserializing)]
    pub email_verified: bool,

    #[validate(length(min = 8, max = 50))]
    password: String,

    #[serde(skip_deserializing, default = "datetime_now")]
    pub created_at: DateTime,
}

impl NewUser {
    pub fn to_hashed_password(mut self) -> Result<Self, AppErr> {
        self.password = argon2_hash(&self.password)?;
        Ok(self)
    }
}

pub fn serialize_option_datetime<S: Serializer>(
    val: &Option<DateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        None => serializer.serialize_none(),
        Some(v) => serializer.serialize_str(&v.to_rfc3339_string()),
    }
}
