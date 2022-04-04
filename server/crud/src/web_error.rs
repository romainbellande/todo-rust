use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
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
    pub fn format(&self) -> FormattedWebError {
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

#[cfg(test)]
mod tests {
    use crate::web_error::{FormattedWebError, WebError};
    use actix_web::http::StatusCode;

    #[test]
    fn format_web_error_with_known_status() {
        let err = WebError {
            code: StatusCode::NOT_FOUND,
            message: String::from("user not found"),
        };

        let expected_err = FormattedWebError {
            code: 404,
            message: String::from("user not found"),
            status: String::from("Not Found"),
        };

        assert_eq!(err.format(), expected_err);
    }

    #[test]
    fn format_web_error_with_unknown_status() {
        let err = WebError {
            code: StatusCode::from_u16(103).unwrap(),
            message: String::from("Early Hints"),
        };

        let expected_err = FormattedWebError {
            code: 103,
            message: String::from("Early Hints"),
            status: String::from("status not supported"),
        };

        assert_eq!(err.format(), expected_err);
    }
}
