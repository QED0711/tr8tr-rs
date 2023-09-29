mod modules;

use modules::transformers;

use polars::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::{DataTransformer, Args, ExecutorFn, FailedTransformationErr};

fn main() {

    // instantiate asset from a csv
    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));

    let mut args_1 = Args::new();
    args_1.insert("out_col".into(), "sma_50".to_string());
    args_1.insert("period".into(), 50);

    let mut args_2 = Args::new();
    args_2.insert("out_col".into(), "sma_100".to_string());
    args_2.insert("period".into(), 100);

    let SMA_50 = transformers::moving_averages::SMA(args_1);
    let SMA_100 = transformers::moving_averages::SMA(args_2);

    asset.set_transformers(vec![
        SMA_50,
        SMA_100,
    ]);

    asset.apply_transformers();
    
    println!("{:?}", asset.df.unwrap());

}
