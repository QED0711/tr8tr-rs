mod modules;
mod environment;

use std::{thread::sleep, time::{Duration, Instant}};
use chrono::{Local, Timelike};
use environment::env;
use modules::transformers;
use modules::triggers;
use modules::notifiers;
// use modules::notifier::Notifier;
// use polars::prelude::*;

use modules::asset::Asset;
// use modules::chart;

// use crate::modules::data_transformer::DataTransformer;
// use crate::modules::transformers::pivot_points::WEEKLY_PIVOT_POINTS;
// use crate::modules::triggers::test;


#[allow(non_snake_case)]
fn main() {
    let environ = env(); 
    let WATCH_DIR = environ.WATCH_DIR;
    let ASSESSMENT_INTERVAL = environ.ASSESSMENT_INTERVAL;
    println!("WATCHING: {WATCH_DIR}");
    println!("ASSESSMENT_INTERVAL: {ASSESSMENT_INTERVAL}");

    // TRANSFORMERS
    let transformer_set = transformers::sets::tfs_001::SET_001();

    // TRIGGERS
    let weekly_pivot_trigger = triggers::sr_bounce::WEEKLY_PIVOT_BOUNCE();
    // let test_buy = triggers::test::TEST_BUY();
    // let test_sell = triggers::test::TEST_SELL();

    // NOTIFIER
    // let mut notifier= notifiers::print_notifier::PRINT();
    let mut notifier = notifiers::ntfy_notifier::NTFY();
    let _ = notifier
        .append_trigger(weekly_pivot_trigger);
        // .append_trigger(test_buy)
        // .append_trigger(test_sell);

    loop {
        let start_time = Instant::now();

        let now = Local::now();
        println!("\tAssessment Beginning - {:02}:{:02}:{:03}", now.minute(), now.second(), now.timestamp_subsec_millis());
        let assets = Asset::from_csv_dir(WATCH_DIR.clone()).unwrap_or(Vec::new());

        for mut asset in assets {
            asset.trim_tail(1); // cut off n rows from the tail
            let _ = asset.transformers.append_transformer_set(transformer_set.clone());
            asset.apply_transformers();
            notifier.evaluate_triggers(&asset);
        }

        let elapsed_time = start_time.elapsed();
        println!("\t\tCompleted in: {}.{} seconds", elapsed_time.as_secs(), elapsed_time.subsec_millis());
        let sleep_duration = std::cmp::max(Duration::from_secs(ASSESSMENT_INTERVAL) - elapsed_time, Duration::from_secs(1));
        sleep(sleep_duration);
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
