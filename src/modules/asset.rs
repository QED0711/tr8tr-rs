use std::collections::HashMap;

use polars::prelude::*;

// mod modules;
use super::data_transformer::DataTransformer;

#[derive(Debug)]
pub struct Asset {
    pub df: Option<DataFrame>,
    pub symbol: Option<String>,
    pub transformers: Vec<DataTransformer>,
    dtypes: HashMap<String, DataType>,
}

/* 
*****************************
********** METHODS ********** 
*****************************
*/

impl Asset {
    /***** UTILITY *****/
    fn new() -> Asset {
        Asset{df: None, symbol: None, transformers: vec![], dtypes: HashMap::new()}
    }

    /***** DATAFRAME *****/
    pub fn from_csv(path: String, symbol: Option<String>) -> Asset {
        let mut a = Asset::new();
        let mut df = CsvReader::from_path(path)
                .unwrap()
                .has_header(true)
                .with_try_parse_dates(true)
                .finish()
                .unwrap();

        a.df = Some(df);
        a.symbol = symbol;
        a
    }

    pub fn trim_tail(&mut self, n: usize) {
        let df = self.df.as_ref().unwrap();
        self.df = Some(df.slice(0, df.height() - n))
    }

    fn get_boxed_value(&self, index: usize, col: &str) -> Result<Box<dyn std::any::Any>, String> {
        match &self.df {
            Some(df) => {
                let col = df.column(col).map_err(|_| format!("Column '{col}' not found"))?;
                match col.dtype() {
                    DataType::Float64 => {
                        Ok(Box::new(col.f64().unwrap().get(index).ok_or("Index out of bounds".to_string())?))
                    }
                    _ => Err("Unsupported data type".to_string())
                }
            },
            None => Err("DataFrame is None".to_string())
        }
    }

    pub fn get_value<T: 'static>(&self, index: usize, col: &str, index_from_end: bool) -> Result<T, String> {
        let target_idx = if index_from_end {self.df.as_ref().unwrap().height() - index} else {index};

        let boxed_val: Box<dyn std::any::Any> = self.get_boxed_value(target_idx, col).unwrap();
        match boxed_val.downcast::<T>() {
            Ok(val) => Ok(*val),
            Err(_) => Err("Failed to downcast".to_string())
        }
    }


    #[allow(dead_code)]
    pub fn to_csv(&self, path: String) {
        // create path to file if it doesn't exit
        let path_obj = std::path::Path::new(&path);
        if let Some(parent) = path_obj.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        let mut file = std::fs::File::create(&path).unwrap();
        CsvWriter::new(&mut file).finish(&mut self.df.clone().unwrap()).unwrap();
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("{:?}", self.df);
    }

    #[allow(dead_code)]
    pub fn shape(&self) -> (usize, usize) {
        let shape = self.df.clone().unwrap().shape();
        return shape
    }


    /***** TRANSFORMERS *****/
    pub fn set_transformers(&mut self, transformers: Vec<DataTransformer>) {
        self.transformers = transformers;
    }

    #[allow(dead_code)]
    pub fn list_transformers(&self) {
        println!("{:?}", self.transformers);
    }

    pub fn apply_transformers(&mut self) {
        let mut transformed_lf: LazyFrame = self.df.clone().unwrap().lazy();
        for transformer in &self.transformers {
            transformed_lf = transformer.apply(transformed_lf).unwrap();
        }
        self.df = Some(
            transformed_lf
                .collect()
                .expect("Failed transforation")
            );
        self.extract_dtypes();
    }

    pub fn extract_dtypes(&mut self) {
        let columns = self.df.as_ref().unwrap().get_column_names();
        let dtypes = self.df.as_ref().unwrap().dtypes();

        for i in 0..columns.len() {
            self.dtypes.insert(columns[i].to_string(), dtypes[i].clone());
        }

        println!("{:?}", self.dtypes);
    }
}

