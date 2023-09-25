mod modules;

// use polars_io::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::DataTransformer;

fn main() {
    // let df = df!["a" => [1, 2, 3], "b" => ["A", "B", "C"]].unwrap();

    let mut asset = Asset::from_csv("~/app/data/AUDUSD.csv".into(), Some("AUDUSD".into()));
    asset.display();

    asset.set_transformers(vec![
        DataTransformer::new("test1".into()),
        DataTransformer::new("test2".into()),
    ]);

    asset.list_transformers();

}
