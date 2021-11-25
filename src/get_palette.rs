use actix_web::{HttpResponse, Responder, get};
use serde::{Serialize};
use crate::error::GetPaletteResponseError;

#[derive(Serialize)]
struct PaletteColour {
    frequency:u64
}

#[get("/to-palette")]
async fn get_palette() -> Result<impl Responder,GetPaletteResponseError> {

    let palette:Vec<PaletteColour> = vec![PaletteColour{frequency:10}];

    Ok(HttpResponse::Ok().json(palette))
}