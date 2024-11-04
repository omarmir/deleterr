use actix_cors::Cors;
use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web as lab_web;
use deleterr::models::AppData;

mod auth;
mod common;
mod deleterr;
mod overseerr;
mod radarr;
mod sonarr;
mod sonrad;
mod store;
mod tautulli;

/// Creates a session middleware with specific configurations for a
/// cookie-based session store in Rust.
///
/// Arguments:
///
/// * `secret_key`: The `secret_key` parameter is a cryptographic key used for securing and encrypting
/// of client-side session cookie. It is essential for ensuring the integrity and confidentiality of the session
/// information stored in the cookies. Make sure to keep this key secure and never expose it publicly.
///
/// Returns:
///
/// A `SessionMiddleware` instance configured with a
/// `CookieSessionStore` store and the provided `secret_key`. The middleware is further customized with
/// settings such as the cookie name, security options, session lifecycle, content security, and HTTP
/// only flag before being built and returned.
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_state = AppData::default();

    let data = web::Data::new(app_state);

    // We only need to generate this key once. Its the source for encryption.
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
            .configure(crate::deleterr::endpoints::config)
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
