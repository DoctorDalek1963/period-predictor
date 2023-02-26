//! This crate provides a way to predict menstrual cycles based on historical data.

#![feature(array_windows)]

mod parse;
mod plot;
mod predict;

use color_eyre::Result;
use std::fs;

fn main() -> Result<()> {
    self::plot::plot(&fs::read_to_string("raw_data.txt")?)?;
    Ok(())
}
