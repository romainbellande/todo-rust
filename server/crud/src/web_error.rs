use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct FormattedWebError {
  pub code: u16,
  pub message: String,
  pub status: String,
}

pub struct WebError {
  pub code: StatusCode,
  pub message: String,
}

impl WebError {
  pub fn format(&self) -> FormattedWebError{
    let status = self.code.canonical_reason();
    let status = if status.is_some() {
      status.unwrap().to_string()
    } else {
      String::from("status not supported")
    };

    FormattedWebError {
      code: self.code.as_u16(),
      status,
      message: self.message.clone(),
    }
  }
}
