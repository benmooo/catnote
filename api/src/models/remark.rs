use super::datetime_now;
use super::serialize_option_datetime;
use bson::{
    doc,
    oid::ObjectId,
    serde_helpers::{serialize_bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string},
    DateTime,
};
use juniper::{graphql_object, GraphQLInputObject};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::Context;
use crate::err::AppErr;
use crate::err::AppResult;

// This remark model is not for writing to database, but for write to http response
// In other world we should read from db, and deserialize it to this model, and serialize to http
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Remark {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "serialize_object_id_as_hex_string"
    )]
    pub id: ObjectId,

    #[serde(serialize_with = "serialize_object_id_as_hex_string")] // create index
    pub author_id: ObjectId,

    #[serde(serialize_with = "serialize_object_id_as_hex_string")] // create index
    pub note_id: ObjectId,

    // unique
    #[serde(default)]
    pub title: Option<String>,

    // the style of the remark -> has defaults value
    pub body: String,

    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(default, serialize_with = "serialize_option_datetime")]
    pub updated_at: Option<DateTime>,
    #[serde(default, skip_serializing)]
    pub deleted_at: Option<DateTime>,
}

#[graphql_object(Context = Context)]
impl Remark {
    fn id(&self) -> String {
        self.id.to_hex()
    }

    fn author_id(&self) -> String {
        self.author_id.to_hex()
    }

    fn note_id(&self) -> String {
        self.note_id.to_hex()
    }

    fn title(&self) -> &Option<String> {
        &self.title
    }

    fn body(&self) -> &str {
        &self.body
    }

    fn created_at(&self) -> String {
        self.created_at.to_rfc3339_string()
    }

    fn updated_at(&self) -> Option<String> {
        self.updated_at.and_then(|t| Some(t.to_rfc3339_string()))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRemark {
    #[serde(skip_deserializing)]
    pub author_id: ObjectId,
    pub note_id: ObjectId,
    pub title: Option<String>,
    pub body: String,

    #[serde(default = "datetime_now")]
    pub created_at: DateTime,
}

#[derive(GraphQLInputObject, Validate)]
pub struct InputRemark {
    #[validate(length(equal = 24))]
    pub note_id: String,
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub body: String,
}

impl InputRemark {
    pub fn to_new_remark(self, author_id: ObjectId) -> AppResult<NewRemark> {
        Ok(NewRemark {
            author_id,
            note_id: ObjectId::parse_str(&self.note_id).map_err(|_| AppErr::InvalidObjectId)?,
            title: self.title,
            body: self.body,
            created_at: datetime_now(),
        })
    }
}
