use polars::df;
use polars::prelude::*;

pub struct Args {}

type FailedTransformationErr = String;

type ExecutorFn = fn(DataFrame, Args) -> Result<DataFrame, FailedTransformationErr>;

#[derive(Debug)]
pub struct DataTransformer {
    pub name: String,
    pub executor: Option<ExecutorFn>, 
}

impl DataTransformer {
    pub fn new(name: String) -> DataTransformer {
        return DataTransformer{name: name, executor: None}
    }
}