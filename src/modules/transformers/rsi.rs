use polars::{prelude::*, series::ops::NullBehavior};
use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};

#[allow(non_snake_case, dead_code)]
pub fn RSI(args: Args) -> DataTransformer {
    fn rsi_transformer(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        // unpack args
        let in_col: String = args.get("in_col", "close".to_string());
        let out_col: String = args.get("out_col", "rsi".to_string());
        let period: i64 = args.get("period", 14);
        
        // setup moving average parameters
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;

        // apply rsi calculation to lazy frame
        let working_lf = lf
            .with_column(
                col(in_col.as_str())
                    .diff(1i64, NullBehavior::default())
                    .alias("price_change")    
            )
            .with_column(
                when(col("price_change").gt(lit(0)))
                .then(col("price_change"))
                .otherwise(lit(0))
                .alias("gain")
            )
            .with_column(
                when(col("price_change").lt(lit(0)))
                .then(col("price_change") * lit(-1))
                .otherwise(lit(0))
                .alias("loss")
            )
            .with_columns([
                col("gain")
                    .rolling_mean(options.clone())
                    .alias("avg_gain"),
                col("loss")
                    .rolling_mean(options)
                    .alias("avg_loss")
                ]
            )
            .with_column(
                (col("avg_gain") / col("avg_loss")).alias("rs")
            )
            .with_column(
                (lit(100) - (lit(100) / (lit(1) + col("rs")))).alias(out_col.as_str())
            )
            .drop_columns(&["price_change", "gain", "loss", "avg_gain", "avg_loss", "rs"]);
        
        Ok(working_lf)
    }

     DataTransformer::new("RSI".into(), rsi_transformer, Some(args))
}




#[allow(non_snake_case, dead_code)]
pub fn RSI_DIVERGENCE(args: Args) -> DataTransformer {
    fn rsi_divergence_transformer(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {

        let rsi_col: String = args.get("rsi_col", "rsi".to_string());
        let out_col: String = args.get("out_col", "rsi_divergence".to_string());
        let lookback: i64 = args.get("lookback", 5);
        let significance: f64= args.get("significance", 0.02);

        let working_lf = lf
            .with_column(
                when(
                    col("close").lt(col("close").shift(lookback))
                        .and(col("close").lt(col("close").shift(-1)))
                )
                .then(true)
                .otherwise(false)
                .alias("price_lower_low")
            )
            .with_column(
                when(
                    col("close").gt(col("close").shift(lookback))
                        .and(col("close").gt(col("close").shift(-1)))
                )
                .then(true)
                .otherwise(false)
                .alias("price_higher_high")
            )
            .with_column(
                (col(&rsi_col) - col(&rsi_col).shift(lookback))
                .alias("rsi_diff")
            )
            .with_column(
                col("rsi_diff").lt(lit(-1. * significance))
                .alias("rsi_lower_high")
            )
            .with_column(
                (col("rsi_diff").gt(lit(significance)))
                .alias("rsi_higher_low")
            )
            .with_column(
                lit(0)
                .alias(&out_col)
            )
            .with_column(
                when(
                    col("price_lower_low").eq(true)
                    .and(col("rsi_higher_low").eq(true))
                )
                .then(lit(1))
                .otherwise(col(&out_col))
                .alias(&out_col)
            )
            .with_column(
                when(
                    col("price_higher_high").eq(true)
                    .and(col("rsi_lower_high").eq(true))
                )
                .then(lit(-1))
                .otherwise(col(&out_col))
                .alias(&out_col)
            )
            .drop_columns(&["rsi_diff", "price_lower_low", "price_higher_high", "rsi_lower_high", "rsi_higher_low"])
            ;

        Ok(working_lf)

    }
    DataTransformer::new("RSI".into(), rsi_divergence_transformer, Some(args))
}