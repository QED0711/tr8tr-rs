mod modules;

use polars::prelude::*;
use polars::df;

use modules::asset::Asset;
use modules::data_transformer::{DataTransformer, Args, ExecutorFn, FailedTransformationErr};

fn main() {

    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));
    asset.display();

    let mut args = Args::new();
    args.insert("out_col".into(), Box::new("ma_50".to_string()));
    args.insert("period".into(), Box::new(50));

    fn test_handler(df: &DataFrame, args: Args) -> Result<&DataFrame, FailedTransformationErr> {
        let out_col: String = if args.contains_key("out_col") {args["out_col"]} else {"moving_average".to_string()};
        println!("{}", out_col);
        return Ok(df)
    }

    asset.set_transformers(vec![
        DataTransformer::new("test1".into(), test_handler),
    ]);

    asset.list_transformers();

}
