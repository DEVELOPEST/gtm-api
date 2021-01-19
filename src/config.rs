
/// Debug only secret for JWT encoding & decoding.
// const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

// pub const TOKEN_PREFIX: &'static str = "Token ";

pub struct AppState {
    pub secret: Vec<u8>,
}
