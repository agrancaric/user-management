use std::env;

pub fn init_test_environment() {
    env::set_var("JWT_SECRET", "secret");
}
