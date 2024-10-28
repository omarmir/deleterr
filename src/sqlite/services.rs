use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://store.db";
/// Asynchronously creates a database if it does not already exist.
///
/// # Returns
///
/// This function returns `()` upon successful creation of the database.
///
/// # Panics
///
/// This function will panic if there is an error verifying the existence of the database or creating the database.
pub async fn create_database() -> () {
    let db = Sqlite::database_exists(DB_URL).await;

    match db {
        Ok(exists) => {
            if exists {
                println!("Database already exists");
                return;
            }
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("No databse existed, created database"),
                Err(error) => panic!("Unable to create database. Error: {}", error),
            }
        }
        Err(error) => panic!("Unable verify database exists. Error {}", error),
    }
}

pub async fn apply_migrations() -> () {
    let db = SqlitePool::connect(DB_URL).await;

    match db {
        Ok(pool) => {
            let migration = sqlx::migrate!().run(&pool).await;
            if let Err(error) = migration {
                panic!("Unable to apply migrations. Error: {}", error)
            }
        }
        Err(error) => panic!("Unable to connect to database. Error: {}", error),
    }
}
