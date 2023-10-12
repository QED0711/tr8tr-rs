use polars::prelude::*;
use crate::modules::data_transformer::{DataTransformer, TransformerArgs, ExecutorFn};

/**************************************************** ARG TYPES ****************************************************/
#[derive(Debug, Clone)]
pub struct SmaArgs{
    pub in_col: Option<String>,
    pub out_col: Option<String>,
    pub period: Option<i64>
}

#[derive(Debug, Clone)]
pub struct EmaArgs{
    pub in_col: Option<String>,
    pub out_col: Option<String>,
    pub period: Option<i64>
}

#[derive(Debug, Clone)]
pub struct TripleMaTrenendArgs{
    pub in_col: Option<String>,
    pub fast_period: Option<i64>,
    pub medium_period: Option<i64>,
    pub slow_period: Option<i64>,
    pub ma_type: Option<String>,
}

impl TransformerArgs for SmaArgs{}
impl TransformerArgs for EmaArgs{}
impl TransformerArgs for TripleMaTrenendArgs{}

/**************************************************** TRANSFORMERS ****************************************************/

#[allow(non_snake_case, dead_code)]
pub fn SMA(args: SmaArgs) -> DataTransformer<SmaArgs> {
    let sma_transformer: ExecutorFn<SmaArgs> = |lf, args| {
    
        // unpack args
        let in_col= args.in_col.as_deref().unwrap_or("close");
        let out_col= args.out_col.as_deref().unwrap_or("sma");
        let period = args.period.unwrap_or(50);
        
        // setup moving average parameters
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;

        // apply moving averge to lazy frame
        let working_lf = lf
            .with_columns([
                col(in_col)
                    .rolling_mean(options)
                    .alias(out_col)
            ]);
        
        Ok(working_lf)
    };

     DataTransformer::new("SMA".into(), sma_transformer, args)
}

#[allow(non_snake_case, dead_code)]
pub fn EMA(args: EmaArgs) -> DataTransformer<EmaArgs> {
    let ema_transformer: ExecutorFn<EmaArgs> = |lf, args| {
    
        // unpack args
        let in_col= args.in_col.as_deref().unwrap_or("close");
        let out_col= args.out_col.as_deref().unwrap_or("sma");
        let period = args.period.unwrap_or(50);
        
        // calculate weights for exponential moving average
        let alpha = 2.0 / (period as f64 + 1.0);
        let weights: Vec<f64> = (0..period)
            .map(|i| (1.0 - alpha).powi(i as i32) * alpha)
            .collect();
    
        // setup moving average parameters
        let mut options = RollingOptions::default();
        options.window_size = Duration::new(period);
        options.min_periods = period as usize;
        options.weights = Some(weights);

        // apply moving averge to lazy frame
        let working_lf = lf
            .with_columns([
                col(in_col)
                    .rolling_mean(options)
                    .alias(out_col)
            ]);
        
        Ok(working_lf)
    };

     DataTransformer::new("EMA".into(), ema_transformer, args)
}

#[allow(non_snake_case, dead_code)]
pub fn TRIPLE_MA_TREND(args: TripleMaTrenendArgs) -> DataTransformer<TripleMaTrenendArgs> {
    
    let triple_ma_trend_transformer: ExecutorFn<TripleMaTrenendArgs> = |lf, args| {
    
        // unpack args
        let in_col = args.in_col.as_deref().unwrap_or("close");
        let fast_period: i64 = args.fast_period.unwrap_or(50);
        let medium_period: i64 = args.medium_period.unwrap_or(100);
        let slow_period: i64 = args.slow_period.unwrap_or(200);
        let ma_type  = args.ma_type.as_deref().unwrap_or("sma");
        
        // setup moving average parameters
        let mut slow_options = RollingOptions::default();
        slow_options.window_size = Duration::new(slow_period);
        slow_options.min_periods = slow_period as usize;
        
        let mut medium_options = RollingOptions::default();
        medium_options.window_size = Duration::new(medium_period);
        medium_options.min_periods = medium_period as usize;

        let mut fast_options = RollingOptions::default();
        fast_options.window_size = Duration::new(fast_period);
        fast_options.min_periods = fast_period as usize;
        
        if ma_type == "ema" {
            // calculate weights for exponential moving average
            let slow_alpha = 2.0 / (slow_period as f64 + 1.0);
            let slow_weights: Vec<f64> = (0..slow_period)
                .map(|i| (1.0 - slow_alpha).powi(i as i32) * slow_alpha)
                .collect();

            let medium_alpha = 2.0 / (medium_period as f64 + 1.0);
            let medium_weights: Vec<f64> = (0..medium_period)
                .map(|i| (1.0 - medium_alpha).powi(i as i32) * medium_alpha)
                .collect();

            let fast_alpha = 2.0 / (fast_period as f64 + 1.0);
            let fast_weights: Vec<f64> = (0..fast_period)
                .map(|i| (1.0 - fast_alpha).powi(i as i32) * fast_alpha)
                .collect();

            slow_options.weights = Some(slow_weights);
            medium_options.weights = Some(medium_weights);
            fast_options.weights = Some(fast_weights);
        }

        
        // apply moving averge to lazy frame
        let working_lf = lf
            .with_columns([
                col(in_col)
                    .rolling_mean(fast_options)
                    .alias("fast_ma"),
                col(in_col)
                    .rolling_mean(medium_options)
                    .alias("medium_ma"),
                col(in_col)
                    .rolling_mean(slow_options)
                    .alias("slow_ma"),
            ])
            .with_column(lit(0).alias("ma_trend"))
            .with_column(
               when(col("fast_ma").gt(col("medium_ma")).and(col("medium_ma").gt(col("slow_ma"))))
                    .then(lit(1))
                    .otherwise(col("ma_trend"))
                    .alias("ma_trend")
            )
            .with_column(
               when(col("fast_ma").lt(col("medium_ma")).and(col("medium_ma").lt(col("slow_ma"))))
                    .then(lit(-1))
                    .otherwise(col("ma_trend"))
                    .alias("ma_trend")
            )
            ;
        
        Ok(working_lf)
    };

     DataTransformer::new("TRIPLE_MA_TREND".into(), triple_ma_trend_transformer, args)

}