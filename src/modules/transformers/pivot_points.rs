use polars::{prelude::*, series::ops::NullBehavior};
use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};

#[allow(non_snake_case, dead_code)]
pub fn WEEKLY_PIVOT_POINTS(args: Args) -> DataTransformer {
    fn weekly_pivot_points(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        // unpack args
        let time_col: String = args.get("time_col", "time".to_string());
        let out_col_prefix = args.get("out_col_prefix", "".to_string());
        
        let working_lf = lf
            .with_column(
                when(
                    col(&time_col).dt().to_string("%A %H:%M:%S").eq(lit("Sunday 17:00:00"))
                )
                .then(lit(true))
                .otherwise(lit(false))
                .alias("week_start")
            )
            .with_column(
                col("week_start").cumsum(false).alias("week_group")
            )
            .with_column(
                col("high").max().over([col("week_group")]).alias("max_high")
            )
            .with_column(
                col("low").min().over([col("week_group")]).alias("min_low")
            )
            .with_column(
                col("close").last().over([col("week_group")]).alias("last_close")
            )
            .with_column(
                ((col("max_high") + col("min_low") + col("last_close")) / lit(3.))
                .alias(&format!("{}pivot_point", out_col_prefix))
            )
            .with_column(
                ((col("pivot_point") * lit(2.)) - col("max_high"))
                .alias(&format!("{}support_1", out_col_prefix))
            )
            .with_column(
                ((col("pivot_point") - (col("max_high") - col("min_low"))))
                .alias(&format!("{}support_2", out_col_prefix))
            )
            .with_column(
                ((col("pivot_point") * lit(2.)) - col("min_low"))
                .alias(&format!("{}resistance_1", out_col_prefix))
            )
            .with_column(
                ((col("pivot_point") + (col("max_high") - col("min_low"))))
                .alias(&format!("{}resistance_2", out_col_prefix))
            )
            .drop_columns(["week_start", "week_group", "max_high", "min_low", "last_close"])
            ;
        
        Ok(working_lf)
    }

     DataTransformer::new("weekly_pivot_points".into(), weekly_pivot_points, Some(args))
}



