use std::env;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
mod get_palette;
mod error;

const MAX_PAYLOAD_DEFAULT:usize = 10240;
const MAX_PAYLOAD_HARD_CAP:usize = 102400;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Gets max payload from ENV varialbe
    // Uses MAX_PAYLOAD_DEFAULT when unspecifided or set to invalid value
    // Hard capped at 100MiB
    let max_payload:usize = match env::var("MAX_UPLOAD_SIZE"){
        Ok(max) => match max.parse::<usize>(){
            Ok(max) => max.min(MAX_PAYLOAD_HARD_CAP),
            Err(_) => {eprintln!("MAX_UPLOAD_SIZE invalid value! using default of 10MiB"); MAX_PAYLOAD_DEFAULT},
        },
        Err(_) => MAX_PAYLOAD_DEFAULT,
    };
    

    // Start server
    HttpServer::new(move || {
        App::new().app_data(actix_web::web::PayloadConfig::new(1024 * max_payload))
                  .service(get_palette::get_palette)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
