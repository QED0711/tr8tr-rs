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

    pub fn set_triggers(&mut self, triggers: Vec<Trigger>) {
        self.triggers = triggers;
    }

    pub fn evaluate_triggers(&mut self, asset: Asset) {
        for trigger in &self.triggers {
            let resp = trigger.evaluate(&asset);
            if resp.direction != "HOLD" && !self.triggered_ids.iter().any(|el| resp.id.contains(el)) {
                self.triggered_ids.push(resp.id.clone());
                (self.executor)(resp)
            }
        }
    }
}