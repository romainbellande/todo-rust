use actix_web::{middleware::Logger, web, App, HttpServer};
mod todo;
use actix_cors::Cors;
use server::{setup_database, setup_logs, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logs();

    let db = setup_database().await;

    let state: AppState = AppState { db };

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard(); // only dev

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%r %U [%D ms][%s]"))
            .app_data(web::Data::new(state.clone()))
            .service(web::scope("/api/v1").service(todo::controller::controller()))
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}
