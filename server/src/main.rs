#![allow(unused_must_use)]

use actix_web::{web, App, HttpServer, middleware::Logger};
use std::env;
mod todo;
use server::AppState;
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    fast_log::init(fast_log::config::Config::new().console());
    let database_url= env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    log::info!("linking database...");
    let db: DatabaseConnection = Database::connect(database_url).await.expect("can't connect to database");
    let state: AppState = AppState { db };

    log::info!("linking database successful!");
    //router
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %U [%D ms][%s]"))
            //add into actix-web data
            .app_data(web::Data::new(state))
            .service(
        web::scope("/api/v1")
                    .service(todo::controller::controller())
            )
    })
        .bind("0.0.0.0:3001")?
        .run()
        .await
}
