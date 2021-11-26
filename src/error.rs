use actix_web::{HttpResponse, HttpResponseBuilder, error, http::{StatusCode, header}};
use thiserror::Error;



#[derive(Error,Debug)]
pub enum GetPaletteResponseError{
    #[error("{{\"code\":{0}, \"msg\":\"{1}\"}}")]
    BadRequest(u32,String)
}

impl error::ResponseError for GetPaletteResponseError{
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            GetPaletteResponseError::BadRequest(_,_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .body(self.to_string())
    }
}