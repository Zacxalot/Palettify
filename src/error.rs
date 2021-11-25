use actix_web::{HttpResponse, HttpResponseBuilder, error, http::{StatusCode, header}};
use thiserror::Error;



#[derive(Error,Debug)]
pub enum GetPaletteResponseError{
    #[error("{{Bad}}")]
    BadRequest
}

impl error::ResponseError for GetPaletteResponseError{
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            GetPaletteResponseError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .body(self.to_string())
    }
}