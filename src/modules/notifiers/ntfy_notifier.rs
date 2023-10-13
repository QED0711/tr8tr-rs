use std::process::Command;
use:: chrono::prelude::*;
use chrono_tz::US::Eastern;
use crate::modules::notifier::{Notifier, NotifierFn};
use crate::modules::trigger::TriggerResponse;
use crate::environment::env;

pub fn NTFY() -> Notifier {

    let handler: NotifierFn = |trigger: TriggerResponse| {

        let datetime_now = Utc::now().with_timezone(&Eastern).format("%Y-%m-%d %H:%M:%S").to_string();

        let output = Command::new("curl")
            .arg("-H")
            .arg(format!("Title: tr8tRS - {} {}", trigger.direction, trigger.symbol.unwrap()))
            .arg("-H")
            .arg("Priority: urgent")
            .arg("-H")
            .arg(format!("Tags: heavy_dollar_sign,{}", (if trigger.direction == "BUY" {"arrow_up"} else {"arrow_down"})))
            .arg("-d")
            .arg(format!("{}\n{}", trigger.description.unwrap_or("".to_string()), datetime_now))
            .arg(env().NTFY_URL)
            .output()
            .expect("Failed to execute curl command");
        
        if !output.status.success() {
            eprintln!("Failed to send curl post to ntfy: {}", String::from_utf8_lossy(&output.stderr));
        }
    };

    Notifier::new(handler)

}