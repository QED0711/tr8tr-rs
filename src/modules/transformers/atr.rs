
use polars::prelude::*;
use crate::modules::data_transformer::{DataTransformer, ExecutorFn, TransformerArgs};

/**************************************************** ARG TYPES ****************************************************/
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct ATR_Args{
    pub period: Option<i64>,
    pub out_col: Option<String>,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct CandleAtrArgs{
    pub in_col: Option<String>,
    pub high_level: Option<String>,
    pub low_level: Option<String>,
    pub out_col: Option<String>,
}

impl TransformerArgs for ATR_Args{}
impl TransformerArgs for CandleAtrArgs{}

/**************************************************** TRANSFORMERS ****************************************************/

#[allow(non_snake_case, dead_code)]
pub fn ATR(args: ATR_Args) -> DataTransformer<ATR_Args> {
    let atr_transformer: ExecutorFn = |lf, args| {
    
        // unpack args
        let period: i64 = args.period.unwrap_or(50);
        let out_col: String = args.out_col.unwrap_or("atr".to_string());
        
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
    };

     DataTransformer::new("ATR".into(), atr_transformer, args)
}


#[allow(non_snake_case, dead_code)]
pub fn CANDLE_ATR(args: CandleAtrArgs) -> DataTransformer<CandleAtrArgs> {
    let candle_atr_transformer: ExecutorFn<CandleAtrArgs> = |lf, args| {
    
        // unpack args
        let atr_col: String = args.atr_col.unwrap_or("atr".to_string());
        let high_level: String = args.atr_col.unwrap_or("high".to_string());
        let low_level: String = args.atr_col.unwrap_or("low".to_string());
        let out_col: String = args.atr_col.unwrap_or("candle_atr".to_string());


        let working_lf = lf.clone()
            .with_column(
                ((col(&high_level) - col(&low_level)).abs() / col(&atr_col))
                .alias(&out_col)
            )
            ;

        Ok(working_lf)
    };

     DataTransformer::new("ATR".into(), candle_atr_transformer, args)
}