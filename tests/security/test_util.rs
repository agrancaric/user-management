pub fn init_test_environment() {
    std::env::set_var("JWT_SECRET", "SECRET");
}
