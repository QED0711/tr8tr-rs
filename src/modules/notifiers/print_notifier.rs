use crate::modules::notifier::{Notifier, NotifierFn};
use crate::modules::trigger::TriggerResponse;

#[allow(non_snake_case)]
pub fn PRINT() ->  Notifier {


    let handler: NotifierFn = |trig: TriggerResponse| {
        println!("TRIGGER: {:?}", trig);
    };

    Notifier::new(handler)

}