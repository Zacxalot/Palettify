use rocket::response::{Responder, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetPaletteResponseError {
    #[error("Bad Request")]
    BadRequest(u32, String),
}

impl<'r> Responder<'r> for GetPaletteResponseError {
    fn respond_to(self, _request: &rocket::Request) -> rocket::response::Result<'r> {
        Response::build().raw_status(400, "Bad Request").ok()
    }
}
