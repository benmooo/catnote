use std::sync::Arc;

use mongodb::Database;

use crate::{
    mw::AuthInfo,
    repos::{new_remark_loader, NoteRepo, RemarkLoader, RemarkRepo, TagRepo, UserRepo, VCodeRepo},
};

pub struct Context {
    pub db: Arc<Database>,
    pub auth: AuthInfo, // pub note_loader: NoteLoader,
    pub remark_loader: RemarkLoader,
}

impl Context {
    pub fn new(db: Arc<Database>, auth: AuthInfo) -> Self {
        Self {
            db: db.clone(),
            auth,
            remark_loader: new_remark_loader(db),
        }
    }

    pub fn user_repo(&self) -> UserRepo {
        UserRepo::new(self.db.clone())
    }

    pub fn note_repo(&self) -> NoteRepo {
        NoteRepo::new(self.db.clone())
    }

    pub fn vcode_repo(&self) -> VCodeRepo {
        VCodeRepo::new(self.db.clone())
    }

    pub fn tag_repo(&self) -> TagRepo {
        TagRepo::new(self.db.clone())
    }

    pub fn remark_repo(&self) -> RemarkRepo {
        RemarkRepo::new(self.db.clone())
    }
}

// struct AuthInfo {
//     id: String,
//     username: String,
// }

impl juniper::Context for Context {}

pub struct Pool(Arc<Database>);

impl Pool {
    pub fn new(db: Database) -> Self {
        Self(Arc::new(db))
    }

    pub fn user_repo(&self) -> UserRepo {
        UserRepo::new(self.0.clone())
    }

    pub fn vcode_repo(&self) -> VCodeRepo {
        VCodeRepo::new(self.0.clone())
    }

    pub fn note_repo(&self) -> NoteRepo {
        NoteRepo::new(self.0.clone())
    }

    pub fn inner_ref(&self) -> &Arc<Database> {
        &self.0
    }
}
