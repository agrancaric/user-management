use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn init_pool_and_execute_migrations(
    database_url: &str,
    pool_size: usize,
) -> Pool<AsyncPgConnection> {
    execute_migrations(database_url);
    init_pool(database_url, pool_size)
}

pub fn init_pool(database_url: &str, pool_size: usize) -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::new(database_url);

    Pool::builder(manager)
        .max_size(pool_size)
        .build()
        .expect("Unable to create connection pool")
}

// TODO: check if this can be done with async pool instead of opening connection manually
pub fn execute_migrations(database_url: &str) {
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
