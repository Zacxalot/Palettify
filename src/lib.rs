#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

pub mod get_palette;
use crate::get_palette::get_palette_of;

#[shuttle_service::main]
async fn init() -> Result<Rocket<Build>, shuttle_service::Error> {
    println!("Starting Palettify!");

    // Start server
    Ok(rocket::build().mount("/", routes![get_palette_of]))
}
