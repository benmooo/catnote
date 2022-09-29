use std::sync::Arc;

use bson::{doc, oid::ObjectId, Document};

use log::info;
use mongodb::{
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection, Database,
};

use crate::{
    err::AppErr,
    models::{NewUser, User, InsertId},
};

pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // pub async fn users(&self) -> Result<Vec<User>, AppErr> {
    //     let user_collection: Collection<User> = self.db.collection("users");

    //     info!("db_io: find all user without filters!");
    //     let mut cursor = user_collection
    //         .find(doc! {}, FindOptions::builder().limit(20).build())
    //         .await
    //         .unwrap();

    //     let mut users = Vec::new();
    //     while let Some(res) = cursor.next().await {
    //         match res {
    //             Ok(user) => users.push(user),
    //             _ => {}
    //         }
    //     }

    //     Ok(users)
    // }

    pub async fn user_by_email(&self, email: &str) -> Result<User, AppErr> {
        let user_collection: Collection<User> = self.db.collection("users");

        info!("db_io: find user by email & password!");
        user_collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .ok_or(AppErr::ResourceNotFound(format!(
                "user with email: {} not found",
                email
            )))
    }

    pub async fn user_by_id(&self, id: &ObjectId) -> Result<User, AppErr> {
        let user_collection: Collection<User> = self.db.collection("users");

        info!("db_io: find user with id {}!", &id);
        user_collection
            .find_one(doc! {"_id": id }, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .ok_or(AppErr::ResourceNotFound(format!(
                "user with id: {} not found",
                id
            )))
    }

    // create index on { email -> unique, username -> unique }
    pub async fn new_user(&self, new_user: &NewUser) -> Result<InsertId, AppErr> {
        let user_collection: Collection<NewUser> = self.db.collection("users");

        info!("db_io: insert new user");
        let res = user_collection
            .insert_one(new_user, None)
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?;

        let id = res
            .inserted_id
            .as_object_id()
            .ok_or(AppErr::InsertIdIsNotObjectId)?;
        Ok(InsertId::new(id))
    }

    pub async fn update_user_by_email(
        &self,
        email: &str,
        payload: Document,
    ) -> Result<User, AppErr> {
        let user_collection: Collection<User> = self.db.collection("users");

        info!("db_io: update user with payload {}", payload);
        user_collection
            .find_one_and_update(
                doc! {"email": email},
                payload,
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            )
            .await
            .map_err(|err| AppErr::DatabaseError(err.to_string()))?
            .ok_or(AppErr::ResourceNotFound(format!(
                "user with email: {} not found",
                email
            )))
    }
}

pub type UserRepo = UserRepository;
