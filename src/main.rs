use actix_web::{get, middleware::Logger, web, App, HttpServer};

mod deleterr;
mod overseerr;
mod polodb;
mod tautulli;
mod common;
use overseerr::services as os_serv;
use polodb::services as polo_serv;
use tautulli::services as tt_serv;

struct AppData {
    db: polodb_core::Database,
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

    let app_state = AppData {
        db: polo_serv::get_database().expect("Unable to open db. Exiting."),
    };

    let data = web::Data::new(app_state);

    HttpServer::new(move || {
        let app_data = &data;

        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(app_data.clone())
            .service(index)
            .configure(os_serv::config)
            .configure(tt_serv::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
