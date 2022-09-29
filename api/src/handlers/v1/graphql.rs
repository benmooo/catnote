use actix_web::{post, web, Responder, HttpResponse};
use juniper::http::GraphQLRequest;

use crate::{context::{Context, Pool}, graphql::Schema, mw::AuthInfo};


#[post("/")]
pub async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Schema>,
    req: web::Json<GraphQLRequest>,
    auth: AuthInfo,
) -> impl Responder {
    let ctx = Context::new(pool.inner_ref().clone(), auth);
    let res = req.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(res)
}