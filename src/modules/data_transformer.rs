use std::collections::HashMap;
use polars::df;
use polars::prelude::*;

/* 
*************************
***** UTILITY TYPES ***** 
*************************
 */
#[derive(Debug)]
pub struct Args(HashMap<String, Box<dyn std::any::Any>>);

impl Args {
    pub fn new() -> Self {
        Args(HashMap::new())
    }

    pub fn insert<T: 'static>(&mut self, key: String, value: T) {
        self.0.insert(key, Box::new(value));
    }

    pub fn get<T: 'static + Clone>(&self, key: &str, default: T) -> T {
        match self.0.get(key) {
            Some(val) => {
                if let Some(actual_val) = val.downcast_ref::<T>() {
                    actual_val.clone()
                } else {
                    default
                }
            },
            None => default
        }
    }
}

pub type FailedTransformationErr = polars::error::PolarsError;
// ExecutorFn must return a new (owned) DataFrame object to avoid lifetime issues
pub type ExecutorFn = fn(&DataFrame, &Args) -> Result<DataFrame, FailedTransformationErr>;

#[derive(Debug)]
pub struct DataTransformer {
    pub name: String,
    pub executor: ExecutorFn,
    pub args: Args,
}

impl DataTransformer {
    pub fn new(name: String, executor: ExecutorFn, args: Option<Args>) -> DataTransformer {
        DataTransformer {
            name,
            executor,
            args: args.unwrap_or_else(Args::new)
        }
    }

    pub fn apply(&self, df: &DataFrame) -> Result<DataFrame, FailedTransformationErr> {
        (self.executor)(df, &self.args)
    }
}