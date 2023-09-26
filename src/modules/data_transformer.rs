use std::collections::HashMap;
use polars::df;
use polars::prelude::*;

pub type Args = HashMap<String, Box<dyn std::any::Any>>;

pub type FailedTransformationErr = String;

pub type ExecutorFn = fn(&DataFrame, Args) -> Result<&DataFrame, FailedTransformationErr>;

#[derive(Debug)]
pub struct DataTransformer {
    pub name: String,
    pub executor: ExecutorFn, 
}

impl DataTransformer {
    pub fn new(name: String, executor: ExecutorFn) -> DataTransformer {
        return DataTransformer{name: name, executor: executor}
    }
}