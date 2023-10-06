use polars::prelude::PolarsError;

use crate::modules::{trigger::{Trigger, TriggerResponse}, asset::Asset};

struct PriceBand {
    center: f64,
    min: f64,
    max: f64,
}

#[allow(non_snake_case)]
pub fn WEEKLY_PIVOT_BOUNCE() -> Trigger {

    fn weekly_pivot_bounce_executor(asset: &Asset) -> Result<TriggerResponse, PolarsError> {

        let pip_value = match &asset.symbol {
            Some(symbol) => if symbol.contains("JPY")  {0.01} else {0.0001}
            None => 0.0001
        };

        let band_pip_rng = 8.;

        let df = asset.df.as_ref().unwrap();
        let cur = df.tail(1);

        let pivot_point = cur.column("pivot_point").unwrap().f64().unwrap().get(0).unwrap().unwrap();
        let pivot_band: PriceBand = PriceBand{center: pivot_point, min: pivot_point - (pip_value * band_pip_rng), max: pivot_point + (pip_value * band_pip_rng)};


        Ok(TriggerResponse {
            direction: "HOLD".to_string(),
            id: None,
            origin: None,
            description: None,
        })
    }

    Trigger::new("WEEKLY_PIVOT_BOUNCE".to_string(), weekly_pivot_bounce_executor)
}