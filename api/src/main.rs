use actix_web::{middleware, web, App, HttpServer};
use config::Config;
use log::info;
use std::sync::Arc;

use crate::{
    context::{Context, Pool},
    graphql::create_schema,
    handlers::{
        graphiql, graphql as graphql_service, reset_password, send_vcode, signin, signup,
        verify_email, refresh_token, logout,
    },
    services::{EmailService, JwtService}, mw::Auth,
};

mod config;
mod context;
mod err;
mod graphql;
mod handlers;
mod models;
mod repos;
mod services;
mod mw;
mod templates;

#[actix::main]
async fn main() -> anyhow::Result<()> {
    // load .env
    dotenv::dotenv().unwrap();

    // A simple logger that can be configured via environment variables
    // for use with the logging facade exposed by the log crate.
    env_logger::init();

    let cfg = Config::from_env().await;
    let db_client = mongodb::Client::with_options(cfg.mongodb_option)?;
    let db = db_client.database(&std::env::var("DATABASE_NAME")?);
    let pool = web::Data::new(Pool::new(db));
    let schema = Arc::new(create_schema());

    let email_service = web::Data::new(EmailService::from_env());
    let jwt_service = web::Data::new(JwtService::from_env());

    // start http server
    info!("starting HTTP server on http://127.0.0.1:8080");
    info!("GraphiQL playground: http://localhost:8080/graphiql");
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(web::Data::from(schema.clone()))
            .app_data(email_service.clone())
            .app_data(jwt_service.clone())
            .wrap(middleware::Logger::new("%a %t %r %s %b %T"))
            .wrap(middleware::Compress::default())
            .service(graphiql)
            .service(
                web::scope("/api/v1/rest")
                    .service(signup)
                    .service(signin)
                    .service(verify_email)
                    .service(send_vcode)
                    .service(reset_password)
                    .service(refresh_token)
                    .service(logout)
            )
            .service(
                web::scope("/api/v1/graphql")
                    .wrap(Auth::default())
                    .service(graphql_service)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod migration_test;
