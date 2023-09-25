use polars::df;
use polars::prelude::*;

// mod modules;
use super::data_transformer::DataTransformer;

#[derive(Debug)]
pub struct Asset {
    pub df: Option<DataFrame>,
    pub name: Option<String>,
    pub transformers: Vec<DataTransformer>
}

/* 
*****************************
********** METHODS ********** 
*****************************
*/

impl Asset {
    /***** UTILITY *****/
    fn new() -> Asset {
        Asset{df: None, name: None, transformers: vec![]}
    }

    /***** DATAFRAME *****/
    pub fn from_csv(path: String, name: Option<String>) -> Asset {
        let mut a = Asset::new();
        a.df = Some(
            CsvReader::from_path(path)
                .unwrap()
                .has_header(true)
                .finish()
                .unwrap()
        );
        a.name = name;
        a
    }

    pub fn display(&self) {
        println!("{:?}", self.df);
    }

    pub fn shape(&self) -> (usize, usize) {
        let shape = self.df.clone().unwrap().shape();
        return shape
    }


    /***** TRANSFORMERS *****/
    pub fn set_transformers(&mut self, transformers: Vec<DataTransformer>) {
        self.transformers = transformers;
    }

    pub fn list_transformers(&self) {
        println!("{:?}", self.transformers);
    }
}

