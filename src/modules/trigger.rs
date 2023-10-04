use std::io::Error;

use super::asset::Asset;

pub struct TriggerResponse {
    pub id: String,
    pub origin: String,
    pub direction: String,
    pub description: Option<String>,
}

pub type TriggerFn = fn(asset: &Asset) -> Result<TriggerResponse, Error>;

pub struct Trigger {
    executor: TriggerFn,
    origin: String,
}

impl Trigger {
    pub fn new(name: String, executor: TriggerFn) -> Self {
        Self {
            origin: name,
            executor,
        } 
    }

    pub fn evaluate(&self, asset: &Asset) -> TriggerResponse {
        let evaluation = (self.executor)(asset);

        let response = match evaluation {
            Ok(resp) => resp,
            Err(_) => TriggerResponse { 
                id: "".to_string(),
                origin: self.origin.clone(),
                direction: "HOLD".to_string(),
                description: None, 
            }
        };

        response
        
    }
}