//! This module provides a way to plot the data.

use crate::{parse::parse_many_date_pairs, predict::DateDiffDistribution};
use chrono::{Duration, NaiveDate};
use color_eyre::Result;
use plotters::{
    backend::BitMapBackend,
    prelude::{ChartBuilder, IntoDrawingArea},
    series::AreaSeries,
    style::{
        text_anchor::{HPos, Pos, VPos},
        FontTransform, IntoFont, RGBColor, TextStyle, WHITE,
    },
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

    let extra = Duration::weeks(52);

    let root = BitMapBackend::new("periods.png", (2000, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root)
        .margin(40)
        .x_label_area_size(100)
        .y_label_area_size(40)
        .caption("Periods", ("sans-serif", 30))
        .build_cartesian_2d((end - Duration::days(5))..(end + extra), 0.0..0.1)?;

    ctx.configure_mesh()
        .x_desc("Period start date")
        .x_labels(100)
        .x_label_style(
            TextStyle::from(("sans-serif", 10).into_font())
                .transform(FontTransform::Rotate90)
                .pos(Pos::new(HPos::Center, VPos::Top)),
        )
        .x_label_formatter(&|date: &NaiveDate| format!("               {date:?}"))
        .axis_desc_style(("sans-serif", 14))
        .draw()?;

    //ctx.draw_series(data.iter().map(|&date| {
    //Circle::new(
    //(date, 1.0),
    //5,
    //ShapeStyle {
    //color: RED.into(),
    //filled: true,
    //stroke_width: 2,
    //},
    //)
    //}))?;

    let distribution = DateDiffDistribution::new(&data, 12);
    ctx.draw_series(AreaSeries::new(
        start
            .iter_days()
            .take_while(|&date| date <= end + extra)
            .map(|date| (date, distribution.find_probability(date))),
        0.0,
        RGBColor(165, 165, 165),
    ))?;

    root.present()?;

    Ok(())
}
