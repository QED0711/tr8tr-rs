use super::{trigger::{Trigger, TriggerResponse}, asset::Asset};

pub type NotifierFn = fn(TriggerResponse);
pub struct Notifier {
    executor: NotifierFn,
    triggers: Vec<Trigger>,
    triggered_ids: Vec<String>,
}

impl Notifier {
    pub fn new(executor: NotifierFn) -> Self {
        Self {
            executor,
            triggers: vec![],
            triggered_ids: vec![],
        }
    }

    pub fn append_trigger(&mut self, trigger: Trigger) -> &mut Self {
        self.triggers.push(trigger);
        self
    }

    pub fn evaluate_triggers(&mut self, asset: &Asset) {
        for trigger in &self.triggers {
            let resp = trigger.evaluate(&asset);
            println!("{:?}", resp);
            if resp.direction != "HOLD" && !self.triggered_ids.iter().any(|el| resp.id.clone().expect("id was None while direction was BUY or SELL").contains(el)) {
                self.triggered_ids.push(resp.id.clone().expect("id was None while direction was BUY or SELL"));
                (self.executor)(resp)
            }
        }
    }
}