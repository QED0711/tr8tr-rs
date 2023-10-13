mod modules;
mod environment;

use modules::transformers;
use modules::triggers;
use modules::notifiers;
use modules::notifier::Notifier;
use plotters::prelude::*;
use polars::prelude::*;
use clap::Parser;

use modules::asset::Asset;
use modules::chart;

use crate::modules::data_transformer::DataTransformer;
use crate::modules::transformers::pivot_points::WEEKLY_PIVOT_POINTS;
use crate::modules::triggers::test;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "~/app/data/")]
    path: String,
}



fn main() {

    // TRANSFORMERS
    let sma_args = transformers::moving_averages::SmaArgs{
        in_col: None, 
        out_col: Some("sma_50".to_string()), 
        period: Some(50)
    };
    
    let ema_args = transformers::moving_averages::EmaArgs{
        in_col: None, 
        out_col: Some("ema_50".to_string()),
        period: Some(50),
    };

    let ma_trend_args = transformers::moving_averages::TripleMaTrenendArgs{
        in_col: None,
        fast_period: Some(50),
        medium_period: Some(100),
        slow_period: Some(300),
        ma_type: Some("ema".to_string()),
    };

    let rsi_args = transformers::rsi::RsiArgs{
        in_col: None,
        out_col: Some("rsi".to_string()),
        period: Some(14),
    };

    let rsi_divergence_args = transformers::rsi::RsiDivergenceArgs{
        rsi_col: "rsi".to_string(), 
        out_col: Some("rsi_divergence".to_string()),
        lookback: Some(12),
        significance: Some(0.02),
    };

    let pivot_point_args = transformers::pivot_points::WeeklyPivotPointArgs{
        time_col: None,
        out_col_prefix: None,
    };    

    let atr_args = transformers::atr::AtrArgs{
        period: Some(14),
        out_col: Some("atr".to_string()),
    };

    let candle_atr_args = transformers::atr::CandleAtrArgs{
        atr_col: "atr".to_string(),
        low_level: Some("open".to_string()),
        high_level: Some("close".to_string()),
        out_col: Some("candle_atr".to_string()),
    };

    let candle_pattern_args = transformers::candle_patterns::CandlePatternArgs{
        color_out_col: Some("candle_color".to_string()),
        pattern_out_col: Some("candle_pattern".to_string()),
        score_out_col: Some("candle_pattern_score".to_string()),
    };

    let sma_50 = transformers::moving_averages::SMA(sma_args);
    let ema_50 = transformers::moving_averages::EMA(ema_args);
    let triple_ma_trend = transformers::moving_averages::TRIPLE_MA_TREND(ma_trend_args);
    let rsi_14 = transformers::rsi::RSI(rsi_args);
    let rsi_divergence = transformers::rsi::RSI_DIVERGENCE(rsi_divergence_args);
    let pivot_points = transformers::pivot_points::WEEKLY_PIVOT_POINTS(pivot_point_args);
    let atr = transformers::atr::ATR(atr_args);
    let candle_atr = transformers::atr::CANDLE_ATR(candle_atr_args);
    let candle_pattern = transformers::candle_patterns::CANDLE_PATTERN(candle_pattern_args);
    

    // TRIGGERS
    let weekly_pivot_trigger = triggers::sr_bounce::WEEKLY_PIVOT_BOUNCE();
    let test_buy = triggers::test::TEST_BUY();
    let test_sell = triggers::test::TEST_SELL();

    // NOTIFIER
    // let mut notifier= notifiers::print_notifier::PRINT();
    let mut notifier = notifiers::ntfy_notifier::NTFY();
    let _ = notifier
        .append_trigger(weekly_pivot_trigger)
        .append_trigger(test_buy)
        .append_trigger(test_sell);

    // ASSET INITIALIZATION
    let cli_args = Args::parse();
    let assets = Asset::from_csv_dir(cli_args.path).unwrap_or(Vec::new());
    // let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));

    for mut asset in assets {
        asset.trim_tail(1); // cut off n rows from the tail
    

        let _ = asset.transformers 
            .append(sma_50.clone())
            .append(ema_50.clone())
            .append(triple_ma_trend.clone())
            .append(rsi_14.clone())
            .append(rsi_divergence.clone())
            .append(pivot_points.clone())
            .append(atr.clone())
            .append(candle_atr.clone())
            .append(candle_pattern.clone());

        asset.apply_transformers();
        notifier.evaluate_triggers(&asset);


    }


    


    
    
    // for testing:
    // weekly_pivot_trigger.evaluate(&asset);

    
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

    // let df = asset.df.clone().unwrap();
    // println!("{:?}", df.get_column_names());
    // println!("{:?}", df);
    // asset.to_csv("./data/transformed/AUDUSD.csv".to_string());

}
