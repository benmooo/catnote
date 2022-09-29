use std::sync::Arc;

use bson::{doc, oid::ObjectId};
use futures::{TryStreamExt};
use log::info;
use mongodb::{options::FindOptions, Collection, Database};

use crate::{
    err::AppErr,
    models::{InsertId, NewTag, Tag},
};

pub struct TagRepo {
    db: Arc<Database>,
}

impl TagRepo {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // create index on { name -> unique }
    pub async fn new_tag(&self, tag: &NewTag) -> Result<InsertId, AppErr> {
        let tag_collection: Collection<NewTag> = self.db.collection("tags");

        info!("db_io: insert new tag");
        let id = tag_collection
            .insert_one(tag, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .inserted_id
            .as_object_id()
            .ok_or(AppErr::InsertIdIsNotObjectId)?;

        Ok(InsertId::new(id))
    }

    // find by id
    pub async fn tag(&self, id: &ObjectId) -> Result<Tag, AppErr> {
        let tag_collection: Collection<Tag> = self.db.collection("tags");

        info!("db_io: find tag by id: {}!", &id);
        tag_collection
            .find_one(doc! {"_id": id}, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .ok_or(AppErr::ResourceNotFound(format!("tag not found: {}", id)))
    }

    // find by author id
    pub async fn tags_by_author_id(&self, author_id: &ObjectId) -> Result<Vec<Tag>, AppErr> {
        let tag_collection: Collection<Tag> = self.db.collection("tags");

        info!("db_io: find tag by author id: {}!", &author_id);
        tag_collection
            .find(
                doc! {"authorId": author_id},
                FindOptions::builder().limit(100).build(),
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .try_collect()
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))
    }

    // pub async fn del_tag(&self, id: &ObjectId) -> Result<Tag, AppErr> {
    //     let tag_collection: Collection<Tag> = self.db.collection("tags");

    //     info!("db_io: find tag by email & type!");
    //     tag_collection
    //         .delete_one(
    //             doc! { "email": email, "code": code, "usedFor": used_for as i32},
    //             None,
    //         )
    //         .await
    //         .map_err(|err| AppErr::DatabaseError(err.to_string()))?
    //         .ok_or(AppErr::ResourceNotFound(
    //             "verification code not found".to_owned(),
    //         ))
    // }
}
