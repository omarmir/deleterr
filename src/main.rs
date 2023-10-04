use actix_web::{get, middleware::Logger, web, App, HttpServer};
use std::sync::Mutex;

mod overseerr;
use overseerr::services;

struct AppState {
    data: Mutex<Vec<String>>,
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let app_data = web::Data::new(AppState {
            data: Mutex::new(vec![]),
        });

        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
