mod modules;

use modules::transformers;
use modules::triggers;
use plotters::prelude::*;
use polars::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::Args;
use modules::chart;

use crate::modules::data_transformer::DataTransformer;
use crate::modules::transformers::pivot_points::WEEKLY_PIVOT_POINTS;

fn main() {

    // instantiate asset from a csv
    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));
    asset.trim_tail(1); // cut off n rows from the tail

    // let mut sma_args = Args::new();
    let sma_args = transformers::moving_averages::SMA_Args{
        in_col: None, 
        out_col: Some("sma_50".to_string()), 
        period: Some(50)
    };
    // sma_args.insert("out_col".into(), "sma_50".to_string());
    // sma_args.insert("period".into(), 50i64);
    
    // let mut ema_args = Args::new();
    // ema_args.insert("out_col".into(), "ema_50".to_string());
    // ema_args.insert("period".into(), 50i64);

    // let mut ma_trend_args: Args = Args::new();
    // ma_trend_args.insert("fast_period".into(), 50i64);
    // ma_trend_args.insert("medium_period".into(), 100i64);
    // ma_trend_args.insert("slow_period".into(), 200i64);
    // ma_trend_args.insert("ma_type".into(), "ema".to_string());

    // let mut rsi_args = Args::new();
    // rsi_args.insert("out_col".into(), "rsi");
    // rsi_args.insert("period".into(), 14i64);

    // let mut rsi_divergence_args: Args = Args::new();
    // rsi_divergence_args.insert("lookback".into(), 12i64);
    // rsi_divergence_args.insert("significance".into(), 0.02f64);

    // let mut pivot_point_args: Args = Args::new();    
    // pivot_point_args.insert("time_col".into(), "time".to_string());

    // let mut atr_args: Args = Args::new();
    // atr_args.insert("period".into(), 14i64);

    // let mut candle_atr_args: Args = Args::new();
    // candle_atr_args.insert("low_level".into(), "open");
    // candle_atr_args.insert("high_level".into(), "close");

    // let candle_pattern_args = Args::new();

    let sma_50 = transformers::moving_averages::SMA(sma_args);
    // let ema_50 = transformers::moving_averages::EMA(ema_args);
    // let triple_ma_trend = transformers::moving_averages::TRIPLE_MA_TREND(ma_trend_args);
    // let rsi_14 = transformers::rsi::RSI(rsi_args);
    // let rsi_divergence: DataTransformer = transformers::rsi::RSI_DIVERGENCE(rsi_divergence_args);
    // let pivot_points: DataTransformer = transformers::pivot_points::WEEKLY_PIVOT_POINTS(pivot_point_args);
    // let atr: DataTransformer = transformers::atr::ATR(atr_args);
    // let candle_atr: DataTransformer = transformers::atr::CANDLE_ATR(candle_atr_args);
    // let candle_pattern = transformers::candle_patterns::CANDLE_PATTERN(candle_pattern_args);

    asset.set_transformers(vec![
        sma_50,
        // ema_50,
        // triple_ma_trend,
        // rsi_14,
        // rsi_divergence,
        // pivot_points,
        // atr,
        // candle_atr,
        // candle_pattern,
    ]);

    asset.apply_transformers();

    let weekly_pivot_trigger = triggers::sr_bounce::WEEKLY_PIVOT_BOUNCE();

    // for testing:
    weekly_pivot_trigger.evaluate(&asset);

    
    // let _ = chart::plot_columns(
    //     &asset.df.clone().unwrap().tail(Some(1000)), 
    //     vec!["close", "fast_ma", "medium_ma", "slow_ma"], 
    //     vec![&BLACK, &CYAN, &BLUE, &RED],
    //     Some("plots/moving_avgs.png"),
    // );
    // let _ = chart::plot_columns(
    //     &asset.df.clone().unwrap().tail(Some(1000)), 
    //     vec!["ma_trend"], 
    //     vec![&CYAN],
    //     Some("plots/ma_trend.png"),
    // );
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

    let df = asset.df.clone().unwrap();
    println!("{:?}", df.get_column_names());
    println!("{:?}", df);
    // asset.to_csv("./data/transformed/AUDUSD.csv".to_string());

}
