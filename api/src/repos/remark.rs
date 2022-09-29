use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use dataloader::{cached::Loader, BatchFn};
use futures::TryStreamExt;
use futures_util::StreamExt;
use log::info;
use mongodb::{options::FindOptions, Collection, Database};

use crate::{
    err::AppErr,
    models::{InsertId, NewRemark, Remark},
};

pub struct RemarkRepo {
    db: Arc<Database>,
}

impl RemarkRepo {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // create index on { name -> unique }
    pub async fn new_remark(&self, remark: &NewRemark) -> Result<InsertId, AppErr> {
        let remark_collection: Collection<NewRemark> = self.db.collection("remarks");

        info!("db_io: insert new remark");
        let id = remark_collection
            .insert_one(remark, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .inserted_id
            .as_object_id()
            .ok_or(AppErr::InsertIdIsNotObjectId)?;

        Ok(InsertId::new(id))
    }

    // find by id
    pub async fn remark(&self, id: &ObjectId) -> Result<Remark, AppErr> {
        let remark_collection: Collection<Remark> = self.db.collection("remarks");

        info!("db_io: find remark by id: {}!", &id);
        remark_collection
            .find_one(doc! {"_id": id}, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .ok_or(AppErr::ResourceNotFound(format!(
                "remark not found: {}",
                id
            )))
    }

    // find by author id
    pub async fn remarks_by_author_id(&self, author_id: &ObjectId) -> Result<Vec<Remark>, AppErr> {
        let remark_collection: Collection<Remark> = self.db.collection("remarks");

        info!("db_io: find remark by author id: {}!", &author_id);
        remark_collection
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

    // find by note id
    pub async fn remarks_by_note_id(&self, note_id: &ObjectId) -> Result<Vec<Remark>, AppErr> {
        let remark_collection: Collection<Remark> = self.db.collection("remarks");

        info!("db_io: find remark by author id: {}!", &note_id);
        remark_collection
            .find(
                doc! {"noteId": note_id},
                FindOptions::builder().limit(100).build(),
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .try_collect()
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))
    }

    pub async fn remarks_by_note_ids(&self, ids: Vec<&ObjectId>) -> Result<Vec<Remark>, AppErr> {
        let remark_collection: Collection<Remark> = self.db.collection("remarks");

        info!("db_io: find remark by author id: {:?}!", &ids);

        remark_collection
            .find(
                doc! {"noteId": { "$in": &ids }},
                FindOptions::builder().limit(100).build(),
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .try_collect()
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))
    }

    // author id is required because security issue, make sure the user have access to delete the remark
    pub async fn delete_remark(&self, id: &ObjectId, author_id: &ObjectId) -> Result<bool, AppErr> {
        let remark_collection: Collection<Remark> = self.db.collection("remarks");

        info!(
            "db_io: del remark by id: {} & author_id: {}",
            &id, &author_id
        );
        let res = remark_collection
            .update_one(
                doc! { "_id": id, "authorId": author_id},
                doc! {"$set": {"deletedAt": DateTime::now()}},
                None,
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?;

        Ok(res.modified_count == 1)
    }
}
pub type RemarkLoader = Loader<String, Vec<Remark>, RemarkBatcher>;

pub fn new_remark_loader(db: Arc<Database>) -> RemarkLoader {
    Loader::new(RemarkBatcher { db })
}

pub struct RemarkBatcher {
    db: Arc<Database>,
}

impl RemarkBatcher {
    pub async fn remarks_by_note_ids(
        &self,
        ids: Vec<String>,
    ) -> Result<HashMap<String, Vec<Remark>>, AppErr> {
        let remark_collection: Collection<Remark> = self.db.collection("remarks");

        info!("db_io: find remarks by note ids: {:?}", &ids);

        let obj_ids: Vec<ObjectId> = ids
            .iter()
            .map(|id| ObjectId::parse_str(id).unwrap())
            .collect();

        let mut cursor = remark_collection
            .find(
                doc! {"noteId": { "$in": &obj_ids }},
                FindOptions::builder().limit(100).build(),
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?;

        // This this beacause Loader need to return all keys in the return HashMap
        // See https://github.com/cksac/dataloader-rs/issues/18#issuecomment-624722997
        let mut hashmap = ids
            .iter()
            .map(|k| (k.to_owned(), vec![]))
            .collect::<HashMap<String, Vec<Remark>>>();

        while let Some(res) = cursor.next().await {
            if let Ok(remark) = res {
                hashmap
                    .entry(remark.note_id.to_hex())
                    .or_insert_with(|| Vec::<Remark>::new())
                    .push(remark);
            }
        }

        Ok(hashmap)
    }
}

#[async_trait]
impl BatchFn<String, Vec<Remark>> for RemarkBatcher {
    async fn load(&mut self, keys: &[String]) -> HashMap<String, Vec<Remark>> {
        info!("loading batch: {:?}", &keys);

        self.remarks_by_note_ids(keys.into())
            .await
            .unwrap_or(HashMap::new())
    }
}
