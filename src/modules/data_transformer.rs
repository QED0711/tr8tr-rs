use std::collections::HashMap;
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

pub trait TransformerArgs {}

pub type FailedTransformationErr = polars::error::PolarsError;
// ExecutorFn must return a new (owned) DataFrame object to avoid lifetime issues
pub type ExecutorFn<T: TransformerArgs> = fn(LazyFrame, &T) -> Result<LazyFrame, FailedTransformationErr>;

#[derive(Debug)]
pub struct DataTransformer<T: TransformerArgs> {
    pub name: String,
    pub executor: ExecutorFn<T>,
    pub args: T,
}

impl<T:TransformerArgs> DataTransformer<T> {
    pub fn new(name: String, executor: ExecutorFn<T>, args: T ) -> DataTransformer<T> {
        DataTransformer {
            name,
            executor,
            args, 
        }
    }

    pub fn apply(&self, lf: LazyFrame) -> Result<LazyFrame, FailedTransformationErr> {
        (self.executor)(lf, &self.args)
    }
}