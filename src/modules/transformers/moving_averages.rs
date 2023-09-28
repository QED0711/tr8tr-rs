use polars::prelude::*;

use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};


pub fn SMA(args: Args) -> DataTransformer {
    fn sma_transformer(df: DataFrame, args: &Args) -> Result<DataFrame, FailedTransformationErr> {
    
        let in_col: String = args.get("in_col", "close".to_string());
        let out_col: String = args.get("out_col", "ma".to_string());
        let period: i64 = args.get("period", 50);
        
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;

        let working_df = df
            .clone()
            .lazy()
            .with_columns([
                col(in_col.as_str())
                    .rolling_mean(options)
                    .alias(out_col.as_str())
            ])
            .collect()
            .unwrap();
        
        Ok(working_df.to_owned())
    }

     DataTransformer::new("SMA".into(), sma_transformer, Some(args))
}

