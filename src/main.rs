#![feature(proc_macro_hygiene, decl_macro)]
use rocket::routes;

use crate::get_palette::static_rocket_route_info_for_get_palette;
mod error;
mod get_palette;

fn main() {
    println!("Starting Palettify!");

    // Start server
    rocket::ignite().mount("/", routes![get_palette]).launch();
}
