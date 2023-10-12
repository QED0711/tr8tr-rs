use std::collections::HashMap;
use polars::prelude::*;
use core::fmt::Debug;

/* 
*************************
***** UTILITY TYPES ***** 
*************************
 */

pub trait TransformerArgs {}

pub type FailedTransformationErr = polars::error::PolarsError;
// ExecutorFn must return a new (owned) DataFrame object to avoid lifetime issues
pub type ExecutorFn<T> = fn(LazyFrame, &T) -> Result<LazyFrame, FailedTransformationErr>;

#[derive(Debug, Clone)]
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
}

pub trait TransformerObj: Debug {
    fn apply(&self, lf: LazyFrame) -> Result<LazyFrame, FailedTransformationErr>;
}

impl<T: TransformerArgs + Debug> TransformerObj for DataTransformer<T> {
    fn apply(&self, lf: LazyFrame) -> Result<LazyFrame, FailedTransformationErr> {
        (self.executor)(lf, &self.args)
    }
}

#[derive(Debug)]
pub struct TransformerSequence {
    pub transformers: HashMap<usize, Box<dyn TransformerObj>>,
    pub length: usize,
}

impl TransformerSequence {
    pub fn new() -> Self {
        Self {transformers: HashMap::new(), length: 0}
    }    

    pub fn append<T: TransformerArgs + 'static + std::fmt::Debug>(&mut self, transformer: DataTransformer<T>) -> &mut Self {

        self.transformers.insert(self.length, Box::new(transformer));
        self.length += 1;

        self
    }
}