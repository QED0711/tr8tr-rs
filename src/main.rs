mod modules;
mod environment;

use modules::transformers;
use modules::triggers;
use modules::notifiers;
use modules::notifier::Notifier;
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
    let transformer_set = transformers::sets::tfs_001::SET_001();

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
        let _ = asset.transformers.append_transformer_set(transformer_set.clone());

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
