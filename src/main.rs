mod modules;

use polars::prelude::*;
use polars::df;

use modules::asset::Asset;
use modules::data_transformer::{DataTransformer, Args, ExecutorFn, FailedTransformationErr};

fn main() {

    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));
    asset.display();

    let mut args = Args::new();
    args.insert("out_col".into(), "ma_50".to_string());
    // args.insert("period".into(), Box::new(50));

    fn test_handler(df: &DataFrame, args: &Args) -> Result<DataFrame, FailedTransformationErr> {
        let out_col: String = args.get("out_col", "ma".to_string());
        let period: usize = args.get("period", 50);
        println!("out_col: {}", out_col);
        println!("period: {}", period);
        return Ok(df.clone())
    }

    let transformer = DataTransformer::new("test1".into(), test_handler, Some(args));
    asset.set_transformers(vec![
        transformer,
    ]);

    asset.list_transformers();
    let new_df = transformer.apply(&asset.df.unwrap());
    println!("{:?}", new_df);

}
