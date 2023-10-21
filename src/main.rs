use actix_files as fs;
use actix_web::{get, middleware::Logger, web, App, HttpServer};
use deleterr::services as dr_serv;
use overseerr::services as os_serv;
use polodb::services as polo_serv;
use std::path::PathBuf;
use tautulli::services as tt_serv;

mod common;
mod deleterr;
mod overseerr;
mod polodb;
mod tautulli;

struct AppData {
    _db: polodb_core::Database,
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

fn single_page_app() -> Result<fs::NamedFile, actix_web::Error> {
    // 1.
    let path: PathBuf = PathBuf::from("./webapp/dist/index.html");
    Ok(fs::NamedFile::open(path)?)
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
            .service(index)
            //.service(webapp)
            .route("/", web::get().to(single_page_app))
            .service(fs::Files::new("/", "./public").index_file("index.html"))
            .configure(os_serv::config)
            .configure(tt_serv::config)
            .configure(dr_serv::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
