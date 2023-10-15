use std::env::var;
use chrono::{Local, Datelike};
pub enum WatchDirType {
    Static,
    DynamicDate,
} 

#[allow(non_snake_case, non_camel_case_types)]
pub struct _env {
    pub WATCH_DIR_TYPE: WatchDirType,
    pub WATCH_DIR: String,
    pub ASSESSMENT_INTERVAL: u64,
    pub NTFY_URL: String
}

pub fn env() -> _env {

    let watch_dir_type = match var("WATCH_DIR_TYPE").unwrap_or("static".to_string()).as_str() {
        "date" => WatchDirType::DynamicDate, 
        _ => WatchDirType::Static
    };

    let watch_dir = var("WATCH_DIR").expect("'WATCH_DIR' environment variable has not been set");

    let watch_dir = match watch_dir_type {
        WatchDirType::Static => watch_dir,
        WatchDirType::DynamicDate => {
            let today = Local::now();
            let date = format!("{:04}-{:02}-{:02}", today.year(), today.month(), today.day());
            format!("{watch_dir}/{date}").to_string()
        }
    };

    let assessment_interval = var("ASSESSMENT_INTERVAL")
        .unwrap_or("30".to_string())
        .parse::<u64>()
        .unwrap_or(30);

    _env {
        WATCH_DIR_TYPE: watch_dir_type,
        WATCH_DIR: watch_dir,
        ASSESSMENT_INTERVAL: assessment_interval,
        NTFY_URL: var("NTFY_URL").unwrap_or("".to_string()),
    }
}