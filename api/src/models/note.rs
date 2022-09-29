use super::{serialize_option_datetime, Remark};
use bson::{
    doc,
    oid::ObjectId,
    serde_helpers::{serialize_bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string},
    DateTime,
};
use juniper::{graphql_object, GraphQLInputObject};
use mongodb::options::UpdateModifications;
use serde::{ser, Deserialize, Serialize, Serializer};
use validator::Validate;

use crate::Context;
use vec_object_id_as_vec_hex_string::serialize_vec_object_id;

// This note model is not for writing to database, but for write to http response
// In other world we should read from db, and deserialize it to this model, and serialize to http
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "serialize_object_id_as_hex_string"
    )]
    pub id: ObjectId,

    #[serde(serialize_with = "serialize_object_id_as_hex_string")] // create index
    pub author_id: ObjectId,

    pub title: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub body: String,

    #[serde(serialize_with = "serialize_option_object_id_to_string")]
    pub parent_id: Option<ObjectId>,

    // #[serde(default, serialize_with = "serialize_vec_object_id")]
    // pub children: Vec<ObjectId>,

    // links to other notes
    #[serde(default, serialize_with = "serialize_vec_object_id")]
    pub links: Vec<ObjectId>,

    // #[serde(default, serialize_with = "serialize_vec_object_id")] // remark model
    // pub remarks: Vec<ObjectId>,

    #[serde(default, serialize_with = "serialize_vec_object_id")] // tag model
    pub tags: Vec<ObjectId>,

    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(default, serialize_with = "serialize_option_datetime")]
    pub updated_at: Option<DateTime>,
    #[serde(default, skip_serializing)]
    pub deleted_at: Option<DateTime>,
}

#[graphql_object(Context = Context)]
#[graphql(name="note")]
impl Note {
    fn id(&self) -> String {
        self.id.to_hex()
    }

    fn author_id(&self) -> String {
        self.author_id.to_hex()
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn description(&self) -> &Option<String> {
        &self.description
    }

    fn body(&self) -> &str {
        self.body.as_str()
    }

    fn parent_id(&self) -> Option<String> {
        self.parent_id.and_then(|id| Some(id.to_hex()))
    }

    fn link_ids(&self) -> Vec<String> {
        self.links.iter().map(|i| i.to_hex()).collect()
    }

    // fn remark_ids(&self) -> Vec<String> {
    //     self.remarks.iter().map(|i| i.to_hex()).collect()
    // }

    async fn remarks(&self, ctx: &Context) -> Vec<Remark> {
        ctx.remark_loader.load(self.id.to_hex()).await
    }

    fn tag_ids(&self) -> Vec<String> {
        self.tags.iter().map(|i| i.to_hex()).collect()
    }

    fn created_at(&self) -> String {
        self.created_at.to_rfc3339_string()
    }

    fn updated_at(&self) -> Option<String> {
        self.updated_at.and_then(|t| Some(t.to_rfc3339_string()))
    }
}

// need a builder
// this model is used for deserialize from user input -> validate and then convert it into docs and writes to db
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNote {
    // not from user input -> from auth middleware
    #[serde(skip_deserializing)]
    pub author_id: ObjectId,

    // from user inpnut
    // #[validate(length(min = 1))]
    pub title: String,

    // from user input -> default
    #[serde(default)]
    pub description: Option<String>,

    // from user input -> an mark down file  default
    #[serde(default)]
    pub body: String,

    // from user input -> deserialze to object id
    #[serde(default, serialize_with = "serialize_option_string_to_object_id")]
    pub parent_id: Option<String>,

    // This field is not practical, because when we add a note ,we have no idea about the chilren
    // we will implement it by graphql query
    // #[serde(default, serialize_with = "serialize_vec_object_id")]
    // pub children: Vec<ObjectId>,

    // add links ==> graphql muataions
    // #[serde(default, serialize_with = "serialize_vec_object_id")]
    // pub links: Vec<ObjectId>,

    // add remarks ==> graphql muataions
    // #[serde(default, serialize_with = "serialize_vec_object_id")] // remark model
    // pub remarks: Vec<ObjectId>,

    // add tags ==> graphql muataions
    // #[serde(default, serialize_with = "serialize_vec_object_id")] // tag model
    // pub tags: Vec<ObjectId>,
    #[serde(default = "datetime_now")]
    pub created_at: DateTime,
}

pub fn datetime_now() -> DateTime {
    DateTime::now()
}

pub fn serialize_option_string_to_object_id<S: Serializer>(
    val: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        None => serializer.serialize_none(),
        Some(v) => match ObjectId::parse_str(v) {
            Ok(oid) => oid.serialize(serializer),
            Err(_) => Err(ser::Error::custom(format!(
                "cannot convert {} to ObjectId",
                v
            ))),
        },
    }
}

pub fn serialize_option_object_id_to_string<S: Serializer>(
    val: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        None => serializer.serialize_none(),
        Some(v) => serializer.serialize_str(&v.to_hex()),
    }
}

pub mod vec_object_id_as_vec_hex_string {
    use bson::oid::ObjectId;
    use serde::{ser::SerializeSeq, Serializer};
    use std::result::Result;

    /// Serializes a vec<objectid>
    pub fn serialize_vec_object_id<S: Serializer>(
        val: &Vec<ObjectId>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(val.len()))?;
        for id in val {
            seq.serialize_element(&id.to_hex())?;
        }
        seq.end()
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct InputNote {
    pub title: String,

    // from user input -> description
    pub description: Option<String>,

    // from user input -> an mark down file  default
    pub body: String,

    // from user input -> deserialze to object id
    #[validate(length(equal = 12))]
    pub parent_id: Option<String>,
}

impl InputNote {
    pub fn to_new_note(self, author_id: ObjectId) -> NewNote {
        NewNote {
            author_id,
            title: self.title,
            description: self.description,
            body: self.body,
            parent_id: self.parent_id,
            created_at: datetime_now(),
        }
    }
}

#[derive(GraphQLInputObject, Validate)]
pub struct UpdateNotePayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,

    #[validate(length(equal = 24))]
    pub parent_id: Option<String>,

    // add a link [ link id which self is a note ]
    #[validate(length(equal = 24))]
    pub add_link: Option<String>,

    // add a tag [ tag id ]
    #[validate(length(equal = 24))]
    pub add_tag: Option<String>,

    // // add a remark [ remark id ]
    // #[validate(length(equal = 24))]
    // pub add_remark: Option<String>,

    // remove a link [ link id ]
    #[validate(length(equal = 24))]
    pub remove_link: Option<String>,

    // remove a tag [ tag id ]
    #[validate(length(equal = 24))]
    pub remove_tag: Option<String>,

    // remote a remark [ remark id ]
    // #[validate(length(equal = 24))]
    // pub remove_remark: Option<String>,
}

impl Into<UpdateModifications> for UpdateNotePayload {
    fn into(self) -> UpdateModifications {
        // let set = doc! { "$set": {}};
        let mut set = doc! {};
        set.insert("updatedAt", DateTime::now());

        self.title.and_then(|title| set.insert("title", title));
        self.description
            .and_then(|desc| set.insert("description", desc));
        self.body.and_then(|body| set.insert("body", body));
        self.parent_id
            .and_then(|id| set.insert("parentId", ObjectId::parse_str(&id).unwrap()));
        //  {$push:{"skills":"Sports"}}) // add "Sports" element to skills array where _id:3
        //  {$push:{"skills":{$each:["Sports","Acting"]}}}) // adds "Sports" and "Acting" to all arrays

        //  push
        let mut add_to_set = doc! {};
        self.add_link
            .and_then(|id| add_to_set.insert("links", ObjectId::parse_str(&id).unwrap()));
        self.add_tag
            .and_then(|id| add_to_set.insert("tags", ObjectId::parse_str(&id).unwrap()));
        // self.add_remark
        //     .and_then(|id| add_to_set.insert("remarks", ObjectId::parse_str(&id).unwrap()));

        //  {$pop:{"skills":1}}) // removes the last element
        //  {$pop:{"skills":-1}}) //removes the first element
        //  {$pull: { "skills": "GST" }}) // removes "GST"

        let mut pull = doc! {};
        self.remove_link
            .and_then(|id| pull.insert("links", ObjectId::parse_str(&id).unwrap()));
        self.remove_tag
            .and_then(|id| pull.insert("tags", ObjectId::parse_str(&id).unwrap()));
        // self.remove_remark
        //     .and_then(|id| pull.insert("remarks", ObjectId::parse_str(&id).unwrap()));

        let mutation = doc! {
            "$set": set,
            "$addToSet": add_to_set,
            "$pull": pull
        };

        UpdateModifications::Document(mutation)
    }
}
