
use polars::prelude::*;
use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};

#[allow(non_snake_case, dead_code)]
pub fn ATR(args: Args) -> DataTransformer {
    fn atr_transformer(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        // unpack args
        let period: i64 = args.get("period", 50);
        let out_col: String = args.get("out_col", "atr".to_string());
        
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;

        let working_lf = lf.clone()
            .with_columns([
                (col("high") - col("low")).alias("tr1"),
                (col("high") - col("close").shift(1)).abs().alias("tr2"),
                (col("low") - col("close").shift(1)).abs().alias("tr3"),
            ])
            .with_column(lit(0.).alias("true_range"))
            .with_column(
                when(col("tr1").gt_eq(col("tr2")).and(col("tr1").gt_eq("tr3")))
                    .then(col("tr1"))
                    .otherwise(col("true_range"))
                    .alias("true_range")
            )
            .with_column(
                when(col("tr2").gt_eq(col("tr1")).and(col("tr2").gt_eq("tr3")))
                    .then(col("tr2"))
                    .otherwise(col("true_range"))
                    .alias("true_range")
            )
            .with_column(
                when(col("tr3").gt_eq(col("tr1")).and(col("tr3").gt_eq("tr2")))
                    .then(col("tr3"))
                    .otherwise(col("true_range"))
                    .alias("true_range")
            )
            .with_column(
                col("true_range")
                    .rolling_mean(options)
                    .alias(&out_col)
            )
            .drop_columns(&["tr1", "tr2", "tr3", "true_range"])
            ;

        Ok(working_lf)
    }

     DataTransformer::new("ATR".into(), atr_transformer, Some(args))
}