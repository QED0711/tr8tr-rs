mod modules;

use modules::transformers;
use plotters::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::Args;
use modules::chart;

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

    let mut candle_pattern_args = Args::new();

    let sma_50 = transformers::moving_averages::SMA(sma_args);
    let ema_50 = transformers::moving_averages::EMA(ema_args);
    let rsi_14 = transformers::rsi::RSI(rsi_args);
    let candle_pattern = transformers::candle_patterns::CANDLE_PATTERN(candle_pattern_args);

    asset.set_transformers(vec![
        sma_50,
        ema_50,
        rsi_14,
        candle_pattern,
    ]);

    asset.apply_transformers();

    
    let _ = chart::plot_columns(
        &asset.df.clone().unwrap().tail(Some(1000)), 
        vec!["close", "sma_50", "ema_50"], 
        vec![&BLACK, &RED, &BLUE],
        Some("plots/moving_avgs.png"),
    );
    let _ = chart::plot_columns(
        &asset.df.clone().unwrap().tail(Some(1000)), 
        vec!["rsi"], 
        vec![&BLACK],
        Some("plots/rsi.png"),
    );
    println!("{:?}", asset.df.unwrap().head(Some(25)));


}
