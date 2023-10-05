use polars::prelude::PolarsError;

use crate::modules::{trigger::{Trigger, TriggerResponse}, asset::Asset};


pub fn WEEKLY_PIVOT_BOUNCE() -> Trigger {

    fn weekly_pivot_bounce_executor(asset: &Asset) -> Result<TriggerResponse, PolarsError> {

        let pip_value = match &asset.symbol {
            Some(symbol) => if symbol.contains("JPY")  {0.01} else {0.0001}
            None => 0.0001
        };


        Ok(TriggerResponse {
            direction: "HOLD".to_string(),
            id: None,
            origin: None,
            description: None,
        })
    }

    Trigger::new("WEEKLY_PIVOT_BOUNCE".to_string(), weekly_pivot_bounce_executor)
}