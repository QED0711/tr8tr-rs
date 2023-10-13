use std::env::var;

pub struct _env {
    // pub WATCH_DIR: String,
    pub NTFY_URL: String
}

pub fn env() -> _env {
    _env {
        // WATCH_DIR: var("WATCH_DIR").expect("'WATCH_DIR' environment variable has not been set"),
        NTFY_URL: var("NTFY_URL").unwrap_or("".to_string()),
    }
}