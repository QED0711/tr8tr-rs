use polars::prelude::*;
use crate::modules::data_transformer::{DataTransformer, TransformerArgs, ExecutorFn};


#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct CandlePatternArgs{
    pub color_out_col: Option<String>,
    pub pattern_out_col: Option<String>,
    pub score_out_col: Option<String>
}

impl TransformerArgs for CandlePatternArgs {}

#[allow(non_snake_case, dead_code)]
pub fn CANDLE_PATTERN(args: CandlePatternArgs) -> DataTransformer<CandlePatternArgs> {

    let candle_pattern: ExecutorFn<CandlePatternArgs> = |lf: LazyFrame, args| {
    
        // unpack args
        let color_out_col = args.color_out_col.as_deref().unwrap_or("candle_color");
        let pattern_out_col = args.pattern_out_col.as_deref().unwrap_or("candle_pattern");
        let score_out_col = args.score_out_col.as_deref().unwrap_or("candle_pattern_score");

        let working_lf = lf
            .with_column(
           when(col("close").gt(col("open")))
                .then(lit("GREEN"))
                .when(col("close").lt(col("open")))
                .then(lit("RED"))
                .otherwise(lit("GRAY"))
                .alias(color_out_col)
            )

            .with_column(col(color_out_col).shift(1).alias("prev_1_color"))
            .with_column(col(color_out_col).shift(2).alias("prev_2_color"))
            .with_column(col(color_out_col).shift(3).alias("prev_3_color"))

            .with_column(col("open").shift(1).alias("prev_open"))
            .with_column(col("high").shift(1).alias("prev_high"))
            .with_column(col("low").shift(1).alias("prev_low"))
            .with_column(col("close").shift(1).alias("prev_close"))
            .with_column(col("open").shift(2).alias("prev_2_open"))
            .with_column(col("high").shift(2).alias("prev_2_high"))
            .with_column(col("low").shift(2).alias("prev_2_low"))
            .with_column(col("close").shift(2).alias("prev_2_close"))

            .with_column(lit("NO PATTERN").alias(pattern_out_col))

            .with_column( // GREEN ENGULFING
                    when(
                        col(color_out_col).eq(lit("GREEN"))
                            .and(col("prev_1_color").eq(lit("RED")))
                            .and(col("open").lt_eq(col("prev_close")))
                            .and(col("close").gt(col("prev_high")))
                    )
                    .then(lit("GREEN_ENGULFING"))
                    .otherwise(col(pattern_out_col))
                    .alias(pattern_out_col)
            )

            .with_column( // GREEN 3 LINE STRIKE
                when(
                    col(pattern_out_col).eq(lit("GREEN_ENGULFING"))
                        .and(col("prev_2_color").eq(lit("RED")))        
                        .and(col("prev_3_color").eq(lit("RED")))        
                )
                .then(lit("GREEN_THREE_LINE_STRIKE"))
                .otherwise(col(pattern_out_col))
                .alias(pattern_out_col)
            )

            .with_column( // GREEN MORNING STAR
                when(
                    col("prev_1_color").neq(lit("RED"))
                        .and(col("prev_2_color").eq(lit("RED")))
                        .and(col("prev_3_color").eq(lit("RED")))
                        .and(col("prev_high").gt(col("prev_close")))
                        .and(col("prev_low").lt(col("prev_open")))
                        .and(col("open").lt((col("prev_2_open") + col("prev_2_close")) / lit(2)))
                        .and(col("close").gt((col("prev_2_open") + col("prev_2_close")) / lit(2)))
                )
                .then(lit("GREEN_MORNING_STAR"))
                .otherwise(col(pattern_out_col))
                .alias(pattern_out_col)
            )

            .with_column( // RED ENGULFING
                    when(
                        col(color_out_col).eq(lit("RED"))
                            .and(col("prev_1_color").eq(lit("GREEN")))
                            .and(col("open").gt_eq(col("prev_close")))
                            .and(col("close").lt(col("prev_low")))
                    )
                    .then(lit("RED_ENGULFING"))
                    .otherwise(col(pattern_out_col))
                    .alias(pattern_out_col)
            )

            .with_column( // RED 3 LINE STRIKE
                when(
                    col(pattern_out_col).eq(lit("RED_ENGULFING"))
                        .and(col("prev_2_color").eq(lit("GREEN")))        
                        .and(col("prev_3_color").eq(lit("GREEN")))        
                )
                .then(lit("RED_THREE_LINE_STRIKE"))
                .otherwise(col(pattern_out_col))
                .alias(pattern_out_col)
            )

            .with_column( // RED EVENING STAR
                when(
                    col("prev_1_color").neq(lit("GREEN"))
                        .and(col("prev_2_color").eq(lit("GREEN")))
                        .and(col("prev_3_color").eq(lit("GREEN")))
                        .and(col("prev_low").lt(col("prev_close")))
                        .and(col("prev_high").gt(col("prev_open")))
                        .and(col("open").gt((col("prev_2_open") + col("prev_2_close")) / lit(2)))
                        .and(col("close").lt((col("prev_2_open") + col("prev_2_close")) / lit(2)))
                )
                .then(lit("RED_EVENING_STAR"))
                .otherwise(col(pattern_out_col))
                .alias(pattern_out_col)
            )

            .with_column( // score columns
                when(col(pattern_out_col).eq(lit("GREEN_ENGULFING"))).then(lit(0.5))
                .when(col(pattern_out_col).eq(lit("GREEN_THREE_LINE_STRIKE"))).then(lit(1.))
                .when(col(pattern_out_col).eq(lit("GREEN_MORNING_STAR"))).then(lit(1.))
                .when(col(pattern_out_col).eq(lit("RED_ENGULFING"))).then(lit(-0.5))
                .when(col(pattern_out_col).eq(lit("RED_THREE_LINE_STRIKE"))).then(lit(-1.))
                .when(col(pattern_out_col).eq(lit("RED_EVENING_STAR"))).then(lit(-1.))
                .otherwise(lit(0.0))
                .alias(score_out_col)
            )
            
            .drop_columns(&[ // cleanup
                "prev_open", "prev_high", "prev_low", "prev_close", 
                "prev_2_open", "prev_2_high", "prev_2_low", "prev_2_close", 
                "prev_1_color", "prev_2_color", "prev_3_color"
            ])
            ;
        
        
        Ok(working_lf)
    };

     DataTransformer::new("candle_pattern".into(), candle_pattern, args)
}