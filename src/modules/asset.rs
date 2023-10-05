use polars::prelude::*;

// mod modules;
use super::data_transformer::DataTransformer;

#[derive(Debug)]
pub struct Asset {
    pub df: Option<DataFrame>,
    pub symbol: Option<String>,
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
        Asset{df: None, symbol: None, transformers: vec![]}
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
    }
}

