mod modules;

use modules::transformers;

use polars::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::{DataTransformer, Args, ExecutorFn, FailedTransformationErr};

fn main() {

    // instantiate asset from a csv
    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));

    let mut args = Args::new();
    args.insert("out_col".into(), "sma_50".to_string());
    args.insert("period".into(), 50);

    let SMA = transformers::moving_averages::SMA(args);

    asset.set_transformers(vec![
        SMA
    ]);

    asset.apply_transformers();
    
    println!("{:?}", asset.df.unwrap());

}
