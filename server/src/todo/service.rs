use actix_web::{web, http};
use server::{AppState};
use entity::todo::{self};
use validator::Validate;
use sea_orm::{entity::*};
use crud::web_error::WebError;

pub async fn create(data: web::Data<AppState>, todo_dto: web::Json<todo::Model>) -> Result<todo::Model, WebError> {
  let db = &data.db;
  let dto = todo_dto.into_inner();

  match dto.validate() {
    Ok(_) => (),
    Err(e) => return Err(WebError {
      code: http::StatusCode::BAD_REQUEST,
      message: e.to_string()
    })
  };

  let result: todo::Model = todo::ActiveModel {
      name: Set(dto.name.to_owned()),
      ..Default::default()
  }.insert(db)
    .await
    .expect("could not insert todo");

   Ok(result)
}
