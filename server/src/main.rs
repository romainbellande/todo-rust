use actix_web::{web, App, HttpServer, middleware::Logger};
mod todo;
use server::{AppState, setup_database, setup_logs};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logs();

    let db = setup_database().await;

    let state: AppState = AppState { db };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %U [%D ms][%s]"))
            .app_data(web::Data::new(state.clone()))
            .service(
        web::scope("/api/v1")
                    .service(todo::controller::controller())
            )
    })
        .bind("0.0.0.0:3001")?
        .run()
        .await
}
