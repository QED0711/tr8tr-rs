use polars::prelude::*;
use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};

#[allow(non_snake_case, dead_code)]
pub fn SMA(args: Args) -> DataTransformer {
    fn sma_transformer(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        // unpack args
        let in_col: String = args.get("in_col", "close".to_string());
        let out_col: String = args.get("out_col", "sma".to_string());
        let period: i64 = args.get("period", 50);
        
        // setup moving average parameters
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;

        // apply moving averge to lazy frame
        let working_lf = lf
            .with_columns([
                col(in_col.as_str())
                    .rolling_mean(options)
                    .alias(out_col.as_str())
            ]);
        
        Ok(working_lf)
    }

     DataTransformer::new("SMA".into(), sma_transformer, Some(args))
}



#[allow(non_snake_case, dead_code)]
pub fn EMA(args: Args) -> DataTransformer {
    fn ema_transformer(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        // unpack args
        let in_col: String = args.get("in_col", "close".to_string());
        let out_col: String = args.get("out_col", "ema".to_string());
        let period: i64 = args.get("period", 50);
        
        // calculate weights for exponential moving average
        let alpha = 2.0 / (period as f64 + 1.0);
        let weights: Vec<f64> = (0..period)
            .map(|i| (1.0 - alpha).powi(i as i32) * alpha)
            .collect();
    
        // setup moving average parameters
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;
        options.weights = Some(weights);

        // apply moving averge to lazy frame
        let working_lf = lf
            .with_columns([
                col(in_col.as_str())
                    .rolling_mean(options)
                    .alias(out_col.as_str())
            ]);
        
        Ok(working_lf)
    }

     DataTransformer::new("EMA".into(), ema_transformer, Some(args))
}