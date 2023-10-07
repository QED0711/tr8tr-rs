use polars::prelude::PolarsError;

use super::asset::Asset;

pub struct TriggerResponse {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub origin: Option<String>,
    pub direction: String,
    pub description: Option<String>,
}

impl TriggerResponse {
    pub fn hold() -> Self{
        Self {direction: "HOLD".to_string(), symbol: None, id: None, origin: None, description: None}
    }
}

pub type TriggerFn = fn(asset: &Asset) -> Result<TriggerResponse, PolarsError>;

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
                id: None,
                symbol: None,
                origin: None,
                direction: "HOLD".to_string(),
                description: None, 
            }
        };

        response
        
    }
}