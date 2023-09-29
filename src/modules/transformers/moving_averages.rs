use polars::prelude::*;

use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};


pub fn SMA(args: Args) -> DataTransformer {
    fn sma_transformer(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        let in_col: String = args.get("in_col", "close".to_string());
        let out_col: String = args.get("out_col", "ma".to_string());
        let period: i64 = args.get("period", 50);
        
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;

        let working_lf = lf
            // .clone()
            .with_columns([
                col(in_col.as_str())
                    .rolling_mean(options)
                    .alias(out_col.as_str())
            ]);
        
        Ok(working_lf)
    }

     DataTransformer::new("SMA".into(), sma_transformer, Some(args))
}

