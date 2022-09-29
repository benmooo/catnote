use bson::{doc, oid::ObjectId, DateTime};

use futures::TryStreamExt;
use log::info;
use mongodb::{
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Collection, Database,
};
use std::sync::Arc;

use crate::{
    err::{AppErr, AppResult},
    models::{InsertId, NewNote, Note, UpdateNotePayload},
};

pub struct NoteRepository {
    db: Arc<Database>,
}

impl NoteRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn notes_by_author_id(&self, author_id: &ObjectId) -> AppResult<Vec<Note>> {
        let note_collection: Collection<Note> = self.db.collection("notes");

        info!("db_io: find notes by author id: {}", &author_id);
        note_collection
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

    pub async fn new_note(&self, note: &NewNote) -> AppResult<InsertId> {
        let note_collection: Collection<NewNote> = self.db.collection("notes");

        info!("db_io: insert note, title: {}!", &note.title);
        let res = note_collection
            .insert_one(note, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?;

        let id = res
            .inserted_id
            .as_object_id()
            .ok_or(AppErr::InsertIdIsNotObjectId)?;
        Ok(InsertId::new(id))
    }

    pub async fn update_note(
        &self,
        id: &ObjectId,
        payload: UpdateNotePayload,
    ) -> AppResult<Option<Note>> {
        let note_collection: Collection<Note> = self.db.collection("notes");

        info!("db_io: update note, id: {}!", &id);

        let res = note_collection
            .find_one_and_update(
                doc! {"_id": id},
                payload,
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?;

        Ok(res)
    }

    // author id is required because security issue, make sure the user have access to delete the note
    pub async fn delete_note(&self, id: &ObjectId, author_id: &ObjectId) -> Result<bool, AppErr> {
        let note_collection: Collection<Note> = self.db.collection("notes");

        info!("db_io: del note by id: {} & author_id: {}", &id, &author_id);
        let res = note_collection
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

// pub type NoteLoader = Loader<String, Vec<Note>, NoteBatcher>;

// pub fn new_note_loader(db: Arc<Database>) -> NoteLoader {
//     Loader::new(NoteBatcher { db })
// }

pub struct NoteBatcher {
    db: Arc<Database>,
}

impl NoteBatcher {
    // pub async fn notes_by_user_ids(
    //     &self,
    //     ids: Vec<String>,
    // ) -> Result<HashMap<String, Vec<Note>>, AppErr> {
    //     let note_collection: Collection<Note> = self.db.collection("notes");

    //     // let ids = ids.iter()
    //     //     // .filter_map(|s| ObjectId::parse_str(s).ok())
    //     //     .map(|id| bson::Bson::String(id.to_owned()))
    //     //     .collect::<Vec<bson::Bson>>();

    //     info!("db_io: find notes by user ids: {:?}", &ids);
    //     let mut cursor = note_collection
    //         .find(doc! {"author_id": { "$in": &ids }}, None)
    //         .await
    //         .map_err(|err| AppErr::DatabaseError(err.kind.to_string(), err.to_string()))?;

    //     // This this beacause Loader need to return all keys in the return HashMap
    //     // See https://github.com/cksac/dataloader-rs/issues/18#issuecomment-624722997
    //     let mut hashmap = ids
    //         .iter()
    //         .map(|k| (k.to_owned(), vec![]))
    //         .collect::<HashMap<String, Vec<Note>>>();

    //     while let Some(res) = cursor.next().await {
    //         if let Ok(note) = res {
    //             hashmap
    //                 .entry(note.author_id.to_string())
    //                 .or_insert_with(|| Vec::<Note>::new())
    //                 .push(note);
    //         }
    //     }

    //     Ok(hashmap)
    // }
}

// #[async_trait]
// impl BatchFn<String, Vec<Note>> for NoteBatcher {
//     async fn load(&mut self, keys: &[String]) -> HashMap<String, Vec<Note>> {
//         info!("loading batch: {:?}", &keys);

//         self.notes_by_author_ids(keys.into())
//             .await
//             .unwrap_or(HashMap::new())
//     }
// }

pub type NoteRepo = NoteRepository;
