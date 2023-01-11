use std::time::Duration;

use ctor::dtor;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use once_cell::sync::Lazy;
use testcontainers::{
    clients::{self, Cli},
    core::WaitFor,
    images::{self, generic::GenericImage},
    Container,
};
use user_management::common::database::init_pool_and_execute_migrations;

static DOCKER_CLIENT: Lazy<Cli> = Lazy::new(|| clients::Cli::default());

// TODO not sure is there anything smarter, don't want to start the container for each test, but this seems as an antipattern in rust
pub static USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT: Lazy<UserManagementTestContext<'static>> =
    Lazy::new(|| UserManagementTestContext::new());

#[dtor]
fn shutdown() {
    USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.shutdown();
}

pub struct UserManagementTestContext<'a> {
    pub pool: Pool<AsyncPgConnection>,
    pub database: Container<'a, GenericImage>,
}

unsafe impl Send for UserManagementTestContext<'_> {}
unsafe impl Sync for UserManagementTestContext<'_> {}

impl<'a> UserManagementTestContext<'a> {
    pub fn new() -> Self {
        std::env::set_var("JWT_SECRET", "SECRET");
        std::env::set_var("RUST_BACKTRACE", "full");
        env_logger::init();

        let (container, database_url) = start_postgres_container();
        let pool = init_pool_and_execute_migrations(&database_url, 1);

        UserManagementTestContext {
            pool,
            database: container,
        }
    }

    pub fn shutdown(&self) {
        self.pool.close();
        self.database.stop();
    }
}

fn start_postgres_container<'a>() -> (Container<'a, GenericImage>, String) {
    let user = "postgres";
    let password = "postgres";
    let database = "user_management";

    let generic_postgres = images::generic::GenericImage::new("postgres", "14.1-alpine")
        .with_wait_for(WaitFor::Duration {
            length: Duration::from_secs(3),
        })
        .with_env_var("POSTGRES_DB", database)
        .with_env_var("POSTGRES_USER", user)
        .with_env_var("POSTGRES_PASSWORD", password);

    let node = DOCKER_CLIENT.run(generic_postgres);

    let database_url = format!(
        "postgres://{}:{}@localhost:{}/{}",
        user,
        password,
        node.get_host_port_ipv4(5432),
        database
    );

    (node, database_url)
}
