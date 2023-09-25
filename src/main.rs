mod modules;

use modules::asset::Asset;
use modules::data_transformer::DataTransformer;

fn main() {
    let a = Asset{name: "Test".into()};
    let dt = DataTransformer{name:"test".into()};
    println!("{:?}", a);
    println!("{:?}", dt);
}