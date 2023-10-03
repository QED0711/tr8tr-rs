use polars::{prelude::*, series::ops::NullBehavior};
use crate::modules::data_transformer::{DataTransformer, FailedTransformationErr, Args};

#[allow(non_snake_case, dead_code)]
pub fn WEEKLY_PIVOT_POINTS(args: Args) -> DataTransformer {
    fn weekly_pivot_points(lf: LazyFrame, args: &Args) -> Result<LazyFrame, FailedTransformationErr> {
    
        // unpack args
        let time_col: String = args.get("time_col", "time".to_string());
        
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
                col("week_start").cumsum(true).alias("week_group")
            )
            .with_column(
                col("high").max().over([col("week_group")]).alias("max_high")
            )
            .with_column(
                col("low").min().over([col("week_group")]).alias("min_low")
            )
            ;
        
        Ok(working_lf)
    }

     DataTransformer::new("weekly_pivot_points".into(), weekly_pivot_points, Some(args))
}



