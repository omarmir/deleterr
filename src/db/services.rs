use crate::common::models::DeleterrError;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

pub fn init_pool() -> Result<Pool<SqliteConnectionManager>, DeleterrError> {
    let manager = SqliteConnectionManager::file("prunerr.db");
    let pool = r2d2::Pool::new(manager).map_err(|err| {
        DeleterrError::new(err.to_string().as_str())
            .add_prefix("Unable to create a db pool. Error: ")
    })?;

    initialize_tables(pool.clone())?;

    Ok(pool)
}

fn initialize_tables(pool: Pool<SqliteConnectionManager>) -> Result<(), DeleterrError> {
    pool.get()?.execute(
        "CREATE TABLE IF NOT EXISTS users (
                userid INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                password_hash TEXT NOT NULL
            );",
        params![],
    )?;

    pool.get()?.execute(
        "CREATE TABLE IF NOT EXISTS user_exemptions (
                user_exemption_id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                FOREIGN KEY(username) REFERENCES users(username)
            );",
        params![],
    )?;

    pool.get()?.execute(
        "CREATE TABLE IF NOT EXISTS media_exemptions (
                exemption_id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                request_id TEXT NOT NULL,
                FOREIGN KEY(username) REFERENCES users(username)
            );",
        params![],
    )?;

    pool.get()?.execute(
        "CREATE TABLE IF NOT EXISTS services (
                service_id INTEGER PRIMARY KEY,
                host TEXT NOT NULL,
                port TEXT,
                api_key TEXT NOT NULL,
                use_ssl INTEGER,
                base_folder TEXT,
                service_name TEXT NOT NULL
            );",
        params![],
    )?;

    pool.get()?.execute(
        "CREATE INDEX IF NOT EXISTS username_index ON users (username);",
        params![],
    )?;

    pool.get()?.execute(
        "CREATE INDEX IF NOT EXISTS user_exemption_username_index ON user_exemptions (username);",
        params![],
    )?;

    pool.get()?.execute(
        "CREATE INDEX IF NOT EXISTS media_exemption_request_index ON media_exemptions (request_id);",
        params![],
    )?;

    pool.get()?.execute(
        "CREATE INDEX IF NOT EXISTS services_service_name_index ON services (service_name);",
        params![],
    )?;

    Ok(())
}
