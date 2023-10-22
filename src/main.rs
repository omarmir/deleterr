use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web as lab_web;
use deleterr::services as dr_serv;
use overseerr::services as os_serv;
use polodb::services as polo_serv;
use tautulli::services as tt_serv;

mod common;
mod deleterr;
mod overseerr;
mod polodb;
mod tautulli;

struct AppData {
    _db: polodb_core::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_state = AppData {
        _db: polo_serv::get_database().expect("Unable to open db. Exiting."),
    };

    let data = web::Data::new(app_state);

    HttpServer::new(move || {
        let app_data = &data;

        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(app_data.clone())
            .configure(os_serv::config)
            .configure(tt_serv::config)
            .configure(dr_serv::config)
            .service(
                lab_web::spa()
                    .index_file("./webapp/dist/index.html")
                    .static_resources_mount("/assets")
                    .static_resources_location("./webapp/dist/assets")
                    .finish(),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
