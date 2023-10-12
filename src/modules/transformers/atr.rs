
use polars::prelude::*;
use crate::modules::data_transformer::{DataTransformer, ExecutorFn, TransformerArgs};

/**************************************************** ARG TYPES ****************************************************/
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct AtrArgs{
    pub period: Option<i64>,
    pub out_col: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CandleAtrArgs{
    pub atr_col: String,
    pub high_level: Option<String>,
    pub low_level: Option<String>,
    pub out_col: Option<String>,
}

impl TransformerArgs for AtrArgs{}
impl TransformerArgs for CandleAtrArgs{}

/**************************************************** TRANSFORMERS ****************************************************/

#[allow(non_snake_case, dead_code)]
pub fn ATR(args: AtrArgs) -> DataTransformer<AtrArgs> {
    let atr_transformer: ExecutorFn<AtrArgs> = |lf, args| {
    
        // unpack args
        let period: i64 = args.period.unwrap_or(50);
        let out_col = args.out_col.as_deref().unwrap_or("atr");
        
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
        let atr_col  = args.atr_col.as_str();
        let high_level = args.high_level.as_deref().unwrap_or("high");
        let low_level = args.low_level.as_deref().unwrap_or("low");
        let out_col = args.out_col.as_deref().unwrap_or("candle_atr");


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