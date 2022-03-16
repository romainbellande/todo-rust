use actix_web::{Scope, web, HttpResponse, Responder, HttpRequest};
use sea_orm::{entity::*, query::*};
use entity::todo;
use entity::todo::Entity as Todo;
use server::AppState;

pub async fn list(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
  let db = &data.db;

  let todos: Vec<todo::Model> = Todo::find().all(db).await.expect("could not retreive todos");

  HttpResponse::Ok()
    .insert_header(("Content-Type", "text/json;charset=UTF-8"))
    .body(serde_json::json!(todos).to_string())
}

// pub async fn create(rb: web::Data<AppState>, payload: web::Json<Todo>) -> impl Responder {
//   let mut todo_dto = payload.into_inner();
//   todo_dto.id = Some(Uuid::new());
//   rb.save::<Todo>(&todo_dto, &[]).await.expect("an error occured during todo creation");

//   HttpResponse::Created()
//     .insert_header(("Content-Type", "text/json;charset=UTF-8"))
//     .body(serde_json::json!(todo_dto).to_string())
// }

pub fn controller() -> Scope {
  return web::scope("/todos")
    .route("", web::get().to(list))
    // .route("", web::post().to(create));
}
