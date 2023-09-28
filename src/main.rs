mod modules;

use polars::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::{DataTransformer, Args, ExecutorFn, FailedTransformationErr};

fn main() {

    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));
    // asset.display();

    let mut args = Args::new();
    args.insert("out_col".into(), "ma_50".to_string());
    // args.insert("period".into(), Box::new(50));

    fn test_handler(df: DataFrame, args: &Args) -> Result<DataFrame, FailedTransformationErr> {
        let in_col: String = args.get("in_col", "close".to_string());
        let out_col: String = args.get("out_col", "ma".to_string());
        let period: i64 = args.get("period", 50);

        let mut df = df.to_owned(); // take ownership of df within

        let mut options = RollingOptionsImpl::default();
        options.window_size = Duration::parse(format!("{}i", period).as_str()); // string format "{period}i" indicates how many periods to run the window over

        let ma = df.column(&in_col)?
            .rolling_mean(options)?;

        // Add the new moving average column to the DataFrame
        // let renamed_ma = ma.clone().rename(out_col.as_str());
        let working_df = df.with_column(ma)?;
        Ok(working_df.to_owned())
    }

    let transformer = DataTransformer::new("test1".into(), test_handler, Some(args));
    asset.set_transformers(vec![
        transformer,
    ]);

    // asset.list_transformers();
    // let new_df = transformer.apply(&asset.df.unwrap());
    asset.apply_transformers();

    println!("{:?}", asset.df.unwrap());
    // println!("{:?}", new_df);

}
