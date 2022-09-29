use bson::oid::ObjectId;
use juniper::{EmptySubscription, RootNode};

use crate::{
    err::{AppErr, AppResult},
    models::{InputNote, InputTag, InsertId, Note, Tag, UpdateNotePayload, User, InputRemark, Remark},
    Context,
};
use validator::Validate;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn api_version() -> &str {
        "1.0"
    }

    // all query are authenticated!

    // user infomation
    async fn user(ctx: &Context) -> AppResult<User> {
        let id = ctx.auth.object_id()?;
        ctx.user_repo().user_by_id(&id).await
    }

    // notes
    async fn notes(ctx: &Context) -> AppResult<Vec<Note>> {
        let id = ctx.auth.object_id()?;
        ctx.note_repo().notes_by_author_id(&id).await
    }

    async fn tags(ctx: &Context) -> AppResult<Vec<Tag>> {
        let id = ctx.auth.object_id()?;
        ctx.tag_repo().tags_by_author_id(&id).await
    }

    async fn remarks(ctx: &Context) -> AppResult<Vec<Remark>> {
        let id = ctx.auth.object_id()?;
        ctx.remark_repo().remarks_by_author_id(&id).await
    }

}

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn api_version() -> &str {
        "1.0"
    }

    async fn new_note(ctx: &Context, note: InputNote) -> AppResult<InsertId> {
        note.validate()?;
        let id = ctx.auth.object_id()?;

        ctx.note_repo().new_note(&note.to_new_note(id)).await
    }

    async fn update_note(
        ctx: &Context,
        note_id: String,
        payload: UpdateNotePayload,
    ) -> AppResult<Option<Note>> {
        let id = ObjectId::parse_str(&note_id).map_err(|_| AppErr::InvalidObjectId)?;
        payload.validate()?;
        ctx.note_repo().update_note(&id, payload).await
    }

    async fn delete_note(ctx: &Context, id: String) -> AppResult<bool> {
        let id = ObjectId::parse_str(&id).map_err(|_| AppErr::InvalidObjectId)?;
        let user_id = ctx.auth.object_id()?;

        ctx.note_repo().delete_note(&id, &user_id).await
    }

    async fn new_tag(ctx: &Context, tag: InputTag) -> AppResult<InsertId> {
        tag.validate()?;
        let id = ctx.auth.object_id()?;

        ctx.tag_repo().new_tag(&tag.to_new_tag(id)).await
    }

    async fn new_remark(ctx: &Context, remark: InputRemark) -> AppResult<InsertId> {
        remark.validate()?;
        let id = ctx.auth.object_id()?;

        ctx.remark_repo().new_remark(&remark.to_new_remark(id)?).await
    }

    async fn delete_remark(ctx: &Context, id: String) -> AppResult<bool> {
        let id = ObjectId::parse_str(&id).map_err(|_| AppErr::InvalidObjectId)?;
        let user_id = ctx.auth.object_id()?;

        ctx.remark_repo().delete_remark(&id, &user_id).await
    }
}

pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
