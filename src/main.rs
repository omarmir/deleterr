use actix_cors::Cors;
use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web as lab_web;
use deleterr::{endpoints as dr_epts, models::AppData, services as dr_serv};
use overseerr::services as os_serv;
use radarr::services as rd_serv;
use std::sync::OnceLock;
use std::sync::RwLock;
use store::exemptions as st_exempt;
use store::models::PersyManager;
use store::services as st_serv;
use tautulli::services as tt_serv;
mod auth;
mod common;
mod deleterr;
mod overseerr;
mod radarr;
mod store;
mod tautulli;

fn session_middleware(secret_key: Key) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
        .cookie_name(String::from("deleterr"))
        .cookie_secure(false) // TODO: Change this for prod
        .session_lifecycle(BrowserSession::default())
        //.cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}

static PERSY_MANAGER: OnceLock<PersyManager> = OnceLock::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    PERSY_MANAGER
        .set(PersyManager::new())
        .expect("Unable to set store.");

    let app_state = AppData {
        last_update: RwLock::new(None),
        request_cache: RwLock::new(None),
    };

    let data = web::Data::new(app_state);

    let secret_key = Key::generate();

    HttpServer::new(move || {
        let app_data = &data;
        let logger = Logger::default();
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(session_middleware(secret_key.clone()))
            .app_data(app_data.clone())
            .configure(dr_epts::config)
            .service(
                lab_web::spa()
                    .index_file("./webapp/dist/index.html")
                    .static_resources_mount("/assets")
                    .static_resources_location("./webapp/dist/assets")
                    .finish(),
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
