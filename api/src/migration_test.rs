use bson::{doc, DateTime};
use futures::StreamExt;
use mongodb::{
    options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument},
    Collection, Database, IndexModel,
};

use crate::{
    config::Config,
    models::{Note, User, VerificationCode},
};

async fn prepare() -> Database {
    dotenv::dotenv().unwrap();
    let cfg = Config::from_env().await;
    let db_client = mongodb::Client::with_options(cfg.mongodb_option).unwrap();
    db_client.database(&std::env::var("DATABASE_NAME").unwrap())
}

#[actix_web::test]
async fn get_indexes_test() -> anyhow::Result<()> {
    let db = prepare().await;
    let users: Collection<User> = db.collection("users");

    let res = users.list_index_names().await?;
    println!("indexes: {:?}", res);
    Ok(())
}

#[actix_web::test]
async fn drop_indexes_test() -> anyhow::Result<()> {
    let db = prepare().await;

    let users: Collection<User> = db.collection("users");
    let vcode: Collection<VerificationCode> = db.collection("v-codes");
    let notes: Collection<Note> = db.collection("notes");
    let tags: Collection<Note> = db.collection("tags");
    let remarks: Collection<Note> = db.collection("remarks");

    users.drop_index("email_1", None).await?;
    users.drop_index("username_1", None).await?;

    notes.drop_index("authorId_1", None).await?;
    tags.drop_index("name_1", None).await?;
    remarks.drop_index("noteId_1", None).await?;
    vcode.drop_index("expireAt_1", None).await?;

    Ok(())
}

#[actix_web::test]
async fn create_index_test() -> anyhow::Result<()> {
    let db = prepare().await;
    let users: Collection<User> = db.collection("users");
    let vcode: Collection<VerificationCode> = db.collection("v-codes");
    let notes: Collection<Note> = db.collection("notes");
    let tags: Collection<Note> = db.collection("tags");
    let remarks: Collection<Note> = db.collection("remarks");

    // user collection
    // create index with unique property
    users
        .create_indexes(
            vec![
                IndexModel::builder()
                    .keys(doc! {"email": 1 as u32})
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
                IndexModel::builder()
                    .keys(doc! {"username": 1 as u32})
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            ],
            None,
        )
        .await?;

    notes
        .create_index(
            IndexModel::builder()
                .keys(doc! {"authorId": 1 as u32})
                .options(None)
                .build(),
            None,
        )
        .await?;

    tags.create_index(
        IndexModel::builder()
            .keys(doc! {"name": 1 as u32})
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        None,
    )
    .await?;

    remarks
        .create_index(
            IndexModel::builder()
                .keys(doc! {"noteId": 1 as u32})
                .options(None)
                .build(),
            None,
        )
        .await?;

    vcode
        .create_index(
            IndexModel::builder()
                .keys(doc! {"expireAt": 1 as u32})
                .options(
                    IndexOptions::builder()
                        .expire_after(std::time::Duration::ZERO)
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    Ok(())
}

#[actix_web::test]
async fn aggregation_test() -> anyhow::Result<()> {
    let db = prepare().await;
    let users: Collection<User> = db.collection("users");

    // find one and update -> and returns the updated one
    let pipline = vec![
        // find
        doc! {
            "$match": { "email": "1937183051@qq.com" }
        },
        // update
        doc! {
            "$set": { "email_verified": false }
        },
        // return the updated one
        doc! {
            "$match": { "email": "1937183051@qq.com"}
        },
    ];

    let mut res = users.aggregate(pipline, None).await?;

    while let Some(doc) = res.next().await {
        match doc {
            Ok(doc) => {
                println!("{}", doc)
            }
            _ => {}
        }
    }

    Ok(())
}

#[actix_web::test]
async fn find_one_and_update_test() -> anyhow::Result<()> {
    let db = prepare().await;
    let users: Collection<User> = db.collection("users");

    let res = users
        .find_one_and_update(
            doc! { "email": "1937183051@qq.com" },
            doc! {"$set": { "email_verified": false, "updated_at": DateTime::now() }},
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        )
        .await?;

    println!("res: {:?}", serde_json::to_string(&res.unwrap()));

    Ok(())
}
