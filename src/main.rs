use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web as lab_web;
use deleterr::{endpoints as dr_serv, models::AppData};
use overseerr::services as os_serv;
use radarr::services as rd_serv;
use std::sync::RwLock;
use store::exemptions as st_exempt;
use store::services as st_serv;
use tautulli::services as tt_serv;

mod common;
mod deleterr;
mod overseerr;
mod radarr;
mod store;
mod tautulli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_state = AppData {
        last_update: RwLock::new(None),
        request_cache: RwLock::new(None),
    };

    let data = web::Data::new(app_state);

    HttpServer::new(move || {
        let app_data = &data;
        let logger = Logger::default();
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(app_data.clone())
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
