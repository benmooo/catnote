use actix_web::{get, Responder, HttpResponse};
use juniper::http::graphiql::graphiql_source;

#[get("/graphiql")]
pub async fn graphiql() -> impl Responder {
    let html = graphiql_source("/api/v1/graphql/", None);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}