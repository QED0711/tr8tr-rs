use polars::{prelude::*, series::ops::NullBehavior};
use crate::modules::data_transformer::{DataTransformer, ExecutorFn, TransformerArgs};

/**************************************************** ARG TYPES ****************************************************/
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct RsiArgs{
    pub in_col: Option<String>,
    pub out_col: Option<String>,
    pub period: Option<i64>,
}

pub struct RsiDivergenceArgs{
    pub in_col: Option<String>,
    pub out_col: Option<String>,
    pub lookback: Option<i64>,
    pub significance: Option<i64>,
}


impl TransformerArgs for RsiArgs{}
impl TransformerArgs for RsiDivergenceArgs{}

/**************************************************** TRANSFORMERS ****************************************************/
#[allow(non_snake_case, dead_code)]
pub fn RSI(args: RsiArgs) -> DataTransformer<RsiArgs> {
    let rsi_transformer: ExecutorFn<RsiArgs> = |lf, args| {
    
        // unpack args
        let in_col = args.in_col.unwrap_or("close".to_string());
        let out_col = args.out_col.unwrap_or("rsi".to_string());
        let period = args.period.unwrap_or(14);
        
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
    };

     DataTransformer::new("RSI".into(), rsi_transformer, Some(args))
}




#[allow(non_snake_case, dead_code)]
pub fn RSI_DIVERGENCE(args: RsiDivergenceArgs) -> DataTransformer<RsiDivergenceArgs> {
    let rsi_divergence_transformer: ExecutorFn<RsiDivergenceArgs> = |lf, args| {

        let rsi_col: String = args.rsi_col.unwrap_or("rsi".to_string());
        let out_col: String = args.out_col.unwrap_or("rsi_divergence".to_string());
        let lookback: i64 = args.lookback.unwrap_or(5);
        let significance: f64 = args.significance.unwrap_or(0.02);

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

    };
    DataTransformer::new("RSI".into(), rsi_divergence_transformer, args)
}