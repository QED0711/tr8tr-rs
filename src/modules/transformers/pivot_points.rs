use polars::{prelude::*, series::ops::NullBehavior};
use crate::modules::data_transformer::{DataTransformer, ExecutorFn, TransformerArgs};

/**************************************************** ARG TYPES ****************************************************/
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct WeeklyPivotPointArgs{
    pub time_col: Option<String>,
    pub out_col_prefix: Option<String>,
}

impl TransformerArgs for WeeklyPivotPointArgs{}

/**************************************************** TRANSFORMERS ****************************************************/

#[allow(non_snake_case, dead_code)]
pub fn WEEKLY_PIVOT_POINTS(args: WeeklyPivotPointArgs) -> DataTransformer<WeeklyPivotPointArgs> {
    let weekly_pivot_points: ExecutorFn<WeeklyPivotPointArgs> = |lf, args| {
    
        // unpack args
        let time_col = args.time_col.as_deref().unwrap_or("time");
        let out_col_prefix = args.out_col_prefix.as_deref().unwrap_or("");
        
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
    };

     DataTransformer::new("weekly_pivot_points".into(), weekly_pivot_points, args)
}



