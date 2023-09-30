use plotters::prelude::*;
use polars::prelude::*;

pub fn plot_columns(df: &DataFrame, col_names: Vec<&str>, colors: Vec<&RGBColor>, out_path: Option<&str> ) -> Result<(), Box<dyn std::error::Error>> {
    

    let root = BitMapBackend::new(out_path.unwrap_or("plots/plot.png"), (800, 600)).into_drawing_area();
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