mod modules;

use modules::transformers;
use plotters::prelude::*;
use polars::prelude::*;

use modules::asset::Asset;
use modules::data_transformer::{DataTransformer, Args, ExecutorFn, FailedTransformationErr};

fn main() {

    // instantiate asset from a csv
    let mut asset = Asset::from_csv("~/app/data/NZDUSD.csv".into(), Some("NZDUSD".into()));

    let mut args_1 = Args::new();
    args_1.insert("out_col".into(), "sma_50".to_string());
    args_1.insert("period".into(), 50);

    let mut args_2 = Args::new();
    args_2.insert("out_col".into(), "sma_100".to_string());
    args_2.insert("period".into(), 100);

    let SMA_50 = transformers::moving_averages::SMA(args_1);
    let SMA_100 = transformers::moving_averages::SMA(args_2);

    asset.set_transformers(vec![
        SMA_50,
        SMA_100,
    ]);

    asset.apply_transformers();

    // println!("{:?}", asset.df.unwrap());

    // Plotting test
    fn plot_columns(df: &DataFrame, col_names: Vec<&str>, colors: Vec<&RGBColor> ) -> Result<(), Box<dyn std::error::Error>> {
        
        let root = BitMapBackend::new("plots/plot.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut all_data = vec![];

        for &col_name in &col_names {
            let series = df.column(col_name)?;
            let data: Vec<f64> = series.f64()?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();

            all_data.push(data);
        }

        let min_y = all_data.iter().flatten().copied().fold(f64::NAN, f64::min);
        let max_y = all_data.iter().flatten().copied().fold(f64::NAN, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .build_cartesian_2d(0f64..all_data[0].len() as f64, min_y..max_y)?;

        chart.configure_mesh().draw()?;
        
        for (data, color) in all_data.iter().zip(colors.iter()) {
            chart.draw_series(LineSeries::new(
                data.iter().enumerate().map(|(idx, value)| (idx as f64, *value)),
                color
            ))?;
        }
        Ok(())
    }
    
    let _ = plot_columns(
        &asset.df.clone().unwrap().tail(Some(500)), 
        vec!["close", "sma_50", "sma_100"], 
        vec![&RED, &BLUE, &GREEN]
    );
    println!("{:?}", asset.df.unwrap());


}
