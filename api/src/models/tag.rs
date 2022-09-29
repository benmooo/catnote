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

// This note model is not for writing to database, but for write to http response
// In other world we should read from db, and deserialize it to this model, and serialize to http
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "serialize_object_id_as_hex_string"
    )]
    pub id: ObjectId,

    #[serde(serialize_with = "serialize_object_id_as_hex_string")] // create index
    pub author_id: ObjectId,

    // unique
    pub name: String,

    // the style of the tag -> has defaults value
    pub style: Option<String>,

    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(default, serialize_with = "serialize_option_datetime")]
    pub updated_at: Option<DateTime>,
    #[serde(default, skip_serializing)]
    pub deleted_at: Option<DateTime>,
}

#[graphql_object(Context = Context)]
impl Tag {
    fn id(&self) -> String {
        self.id.to_hex()
    }

    fn author_id(&self) -> String {
        self.author_id.to_hex()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn style(&self) -> &Option<String> {
        &self.style
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
pub struct NewTag {
    #[serde(skip_deserializing)]
    pub author_id: ObjectId,
    pub name: String,
    pub style: Option<String>,

    #[serde(default = "datetime_now")]
    pub created_at: DateTime,
}

#[derive(GraphQLInputObject, Validate)]
pub struct InputTag {
    #[validate(length(min = 1))]
    pub name: String,
    pub style: Option<String>,
}

impl InputTag {
    pub fn to_new_tag(self, author_id: ObjectId) -> NewTag {
        NewTag {
            author_id,
            name: self.name,
            style: self.style,
            created_at: datetime_now(),
        }
    }
}
