use actix_web::{App, HttpServer};

mod get_palette;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_palette::get_palette)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
