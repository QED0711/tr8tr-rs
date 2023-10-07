use polars::prelude::*;
use chrono::{TimeZone, Utc};

use crate::modules::{trigger::{Trigger, TriggerResponse}, asset::Asset};

#[derive(Debug)]
struct PriceBand {
    _type: String,
    center: f64,
    min: f64,
    max: f64,
}

impl PriceBand {
    fn interactions(bands: Vec<Self>, n: f64) -> Option<Self> {
        for band in bands{
            if band.interacts_with(n) {
                return Some(band)
            }
        }
        None
    }

    fn interacts_with(&self, n: f64) -> bool {
        self.min <= n && n <= self.max
    }
}

#[allow(non_snake_case)]
pub fn WEEKLY_PIVOT_BOUNCE() -> Trigger {

    fn weekly_pivot_bounce_executor(asset: &Asset) -> Result<TriggerResponse, PolarsError> {

        // Static variable setup
        let symbol = asset.symbol.clone().unwrap();
        let pip_value = match &asset.symbol {
            Some(symbol) => if symbol.contains("JPY")  {0.01} else {0.0001}
            None => 0.0001
        };

        let band_pip_rng = 8.; // n pips +/- the center
        let pip_margin: f64 = pip_value * band_pip_rng;

        let cur_timestamp: i64 = asset.get_value::<i64>("time", 1, true).unwrap();
        let cur_datetime = Utc.timestamp_opt(cur_timestamp / 1_000_000, 0).unwrap(); // convert from microseconds to seconds
        let datetime_str = cur_datetime.format("%Y-%m-%d %H:%M:%S");

        let cur_open: f64 = asset.get_value::<f64>("open", 1, true).unwrap();

        // Support/Resistance Points
        let pivot_point = asset.get_value::<f64>("pivot_point", 1,  true).unwrap();
        let s1: f64 = asset.get_value::<f64>("support_1", 1,true).unwrap();
        let s2: f64 = asset.get_value::<f64>("support_2", 1, true).unwrap();
        let r1: f64 = asset.get_value::<f64>("resistance_1", 1, true).unwrap();
        let r2: f64 = asset.get_value::<f64>("resistance_2", 1, true).unwrap();

        let pivot_band: PriceBand = PriceBand {_type: "pp".to_string(), center: pivot_point, min: pivot_point - pip_margin, max: pivot_point + pip_margin};
        let s1_band: PriceBand = PriceBand {_type: "s1".to_string(), center: s1, min: s1 - pip_margin, max: s1 + pip_margin };
        let s2_band: PriceBand = PriceBand {_type: "s2".to_string(), center: s2, min: s2 - pip_margin, max: s2 + pip_margin };
        let r1_band: PriceBand = PriceBand {_type: "r1".to_string(), center: r1, min: r1 - pip_margin, max: r1 + pip_margin };
        let r2_band: PriceBand = PriceBand {_type: "r2".to_string(), center: r2, min: r2 - pip_margin, max: r2 + pip_margin };

        let price_bands = vec![pivot_band, s1_band, s2_band, r1_band, r2_band];

        // Candle attributes
        let candle_color: String = asset.get_value::<String>("candle_color", 1, true).unwrap();
        let pattern_score: f64 = asset.get_value::<f64>("candle_pattern_score", 1, true).unwrap();

        // BUY Indicators
        if pattern_score == 1.0f64 {
            match PriceBand::interactions(price_bands, cur_open) {
                Some(band) => {
                    return Ok(TriggerResponse {
                            direction: "BUY".to_string(),
                            symbol: Some(symbol.clone()),
                            id: Some(format!("{symbol} - {datetime_str} - weekly_pivot_bounce").to_string()),
                            origin: Some("WEEKLY_PIVOT_BOUNCE".to_string()),
                            description: Some(format!("bounce from {}", band._type).to_string()),
                        })
                }
                None => {
                    return Ok(TriggerResponse::hold())
                }
            }
        }

        Ok(TriggerResponse::hold())
        
    }

    Trigger::new("WEEKLY_PIVOT_BOUNCE".to_string(), weekly_pivot_bounce_executor)
}