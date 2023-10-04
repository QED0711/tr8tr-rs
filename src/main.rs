mod modules;

use modules::transformers;
use plotters::prelude::*;
use polars::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::Args;
use modules::chart;

use crate::modules::data_transformer::DataTransformer;

fn main() {

    // instantiate asset from a csv
    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));

    let mut sma_args = Args::new();
    sma_args.insert("out_col".into(), "sma_50".to_string());
    sma_args.insert("period".into(), 50i64);
    
    let mut ema_args = Args::new();
    ema_args.insert("out_col".into(), "ema_50".to_string());
    ema_args.insert("period".into(), 50i64);

    let mut rsi_args = Args::new();
    rsi_args.insert("out_col".into(), "rsi");
    rsi_args.insert("period".into(), 14i64);

    let mut rsi_divergence_args: Args = Args::new();
    rsi_divergence_args.insert("lookback".into(), 12i64);
    rsi_divergence_args.insert("significance".into(), 0.02f64);

    let mut pivot_point_args: Args = Args::new();    
    pivot_point_args.insert("time_col".into(), "time".to_string());

    let mut atr_args: Args = Args::new();
    atr_args.insert("period".into(), 14i64);

    let candle_pattern_args = Args::new();

    let sma_50 = transformers::moving_averages::SMA(sma_args);
    let ema_50 = transformers::moving_averages::EMA(ema_args);
    let rsi_14 = transformers::rsi::RSI(rsi_args);
    let rsi_divergence: DataTransformer = transformers::rsi::RSI_DIVERGENCE(rsi_divergence_args);
    let pivot_points: DataTransformer = transformers::pivot_points::WEEKLY_PIVOT_POINTS(pivot_point_args);
    let atr: DataTransformer = transformers::atr::ATR(atr_args);
    let candle_pattern = transformers::candle_patterns::CANDLE_PATTERN(candle_pattern_args);

    asset.set_transformers(vec![
        sma_50,
        ema_50,
        rsi_14,
        rsi_divergence,
        pivot_points,
        atr,
        candle_pattern,
    ]);

    asset.apply_transformers();

    
    let _ = chart::plot_columns(
        &asset.df.clone().unwrap().tail(Some(1000)), 
        vec!["close", "pivot_point", "support_1", "resistance_1"], 
        vec![&BLACK, &CYAN, &BLUE, &RED],
        Some("plots/moving_avgs.png"),
    );
    let _ = chart::plot_columns(
        &asset.df.clone().unwrap().tail(Some(1000)), 
        vec!["atr"], 
        vec![&CYAN],
        Some("plots/atr.png"),
    );
    // let _ = chart::plot_columns(
    //     &asset.df.clone().unwrap().tail(Some(500)), 
    //     vec!["rsi"], 
    //     vec![&BLACK],
    //     Some("plots/rsi.png"),
    // );
    // let _ = chart::plot_columns(
    //     &asset.df.clone().unwrap().tail(Some(500)), 
    //     vec!["rsi_divergence"], 
    //     vec![&BLACK],
    //     Some("plots/rsi_divergence.png"),
    // );

    println!("{:?}", asset.df.clone().unwrap());
    asset.to_csv("./data/transformed/AUDUSD.csv".to_string());
    // println!("{:?}", asset.df.clone().unwrap().columns(&["candle_pattern", "candle_pattern_score"]));
    // println!("{:?}", asset.df.unwrap().group_by("candle_pattern").agg(&[col("candle_pattern").count().alias("candle_pattern_count")]));


}
