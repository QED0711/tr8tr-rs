use crate::modules::{trigger::{Trigger, TriggerResponse}, asset::Asset};
use polars::prelude::*;

pub fn TEST_BUY() -> Trigger {
    fn test_buy(asset: &Asset) -> Result<TriggerResponse, PolarsError> {
        let symbol = asset.symbol.clone().unwrap();
        return Ok(TriggerResponse{
            direction: "BUY".to_string(),
            symbol: Some(symbol.clone()),
            id: Some("BUY".to_string()),
            origin: Some("Test BUY Trigger".to_string()),
            description: None,
        });
    }

    Trigger::new("TEST BUY".to_string(), test_buy)
}


pub fn TEST_SELL() -> Trigger {
    fn test_sell(asset: &Asset) -> Result<TriggerResponse, PolarsError> {
        let symbol = asset.symbol.clone().unwrap();
        return Ok(TriggerResponse{
            direction: "SELL".to_string(),
            symbol: Some(symbol.clone()),
            id: Some("SELL".to_string()),
            origin: Some("Test SELL Trigger".to_string()),
            description: None,
        });
    }

    Trigger::new("TEST BUY".to_string(), test_sell)
}