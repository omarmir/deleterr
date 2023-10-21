use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
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
            .service(
                fs::Files::new("/", "webapp/dist")
                    .show_files_listing()
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .configure(os_serv::config)
            .configure(tt_serv::config)
            .configure(dr_serv::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
