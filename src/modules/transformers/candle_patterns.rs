use polars::{prelude::*, series::ops::NullBehavior};
use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};

#[allow(non_snake_case, dead_code)]
pub fn CANDLE_PATTERN(args: Args) -> DataTransformer {
    fn candle_pattern(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        // unpack args
        // let in_col: String = args.get("in_col", "close".to_string());
        let color_out_col: String = args.get("color_out_col", "candle_color".to_string());
        let pattern_out_col: String = args.get("pattern_out_col", "candle_pattern".to_string());
        let score_out_col: String = args.get("score_out_col", "candle_patthern_score".to_string());
        // let period: i64 = args.get("period", 14);

        let working_lf = lf
            .with_column(
           when(col("close").gt(col("open")))
                .then(lit("GREEN"))
                .when(col("close").lt(col("open")))
                .then(lit("RED"))
                .otherwise(lit("GRAY"))
                .alias(color_out_col.as_str())
            )
            .with_column(col(color_out_col.as_str()).shift(1).alias("prev_1_color"))
            .with_column(col(color_out_col.as_str()).shift(2).alias("prev_2_color"))
            .with_column(col(color_out_col.as_str()).shift(3).alias("prev_3_color"))
            .with_column(lit("NO PATTERN").alias(pattern_out_col.as_str()))
            .with_column(
                    when(
                        col(color_out_col.as_str()).eq(lit("GREEN"))
                            .and(col("prev_1_color").eq(lit("RED")))
                            // TODO: complete green engulfing logic here
                    )
                    .then(lit("GREEN_ENGULFING"))
                    .otherwise(col(pattern_out_col.as_str()))
                    .alias(pattern_out_col.as_str())
            )
            ;
        
        
        Ok(working_lf)
    }

     DataTransformer::new("candle_pattern".into(), candle_pattern, Some(args))
}