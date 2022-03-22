use actix_web::{HttpResponse, Responder, HttpResponseBuilder};
use serde::Serialize;
use crate::web_error::WebError;

const DEFAULT_HEADER: (&str, &str) = ("Content-Type", "text/json;charset=UTF-8");

pub fn handle_success<Model: Serialize>(mut builder: HttpResponseBuilder, data: Model) -> HttpResponse {
    builder
        .insert_header(DEFAULT_HEADER)
        .body(serde_json::json!(data).to_string())
}

pub fn handle_error(err: WebError) -> HttpResponse {
    HttpResponse::build(err.code)
        .insert_header(DEFAULT_HEADER)
        .body(serde_json::json!(err.format()).to_string())
}

pub fn create<Model: Serialize>(result: Result<Model, WebError>) -> impl Responder {
  match result {
    Ok(data) => handle_success::<Model>(HttpResponse::Created(), data),
    Err(err) => handle_error(err)
  }
}
