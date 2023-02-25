//! This module provides a way to plot the data.

use crate::parse::parse_many_date_pairs;
use chrono::{Duration, NaiveDate};
use color_eyre::Result;
use plotters::{
    backend::BitMapBackend,
    prelude::{ChartBuilder, Circle, IntoDrawingArea},
    style::{ShapeStyle, RED, WHITE},
};

/// Get the start dates of all the periods in the given input.
fn get_data(input: &str) -> Result<Vec<NaiveDate>> {
    let parsed_dates = parse_many_date_pairs(input).unwrap().1;
    let period_dates: Vec<_> = parsed_dates.into_iter().map(|(start, _)| start).collect();

    Ok(period_dates)
}

/// Plot the data and generate `periods.png`.
pub fn plot(input: &str) -> Result<()> {
    let data = get_data(input)?;
    let start = *data.first().unwrap();
    let end = *data.last().unwrap();

    let root = BitMapBackend::new("periods.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root)
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(20)
        .caption("Periods", ("sans-serif", 20))
        .build_cartesian_2d(
            (start - Duration::days(10))..(end + Duration::days(10)),
            0.0..1.2,
        )?;

    ctx.configure_mesh().x_desc("Period start date").draw()?;
    ctx.draw_series(data.into_iter().map(|date| {
        Circle::new(
            (date, 1.0),
            5,
            ShapeStyle {
                color: RED.into(),
                filled: true,
                stroke_width: 2,
            },
        )
    }))?;

    root.present()?;

    Ok(())
}
