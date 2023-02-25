//! This module contains functions to parse thing like dates.

use chrono::NaiveDate;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult, Parser,
};

/// Concisely create a NaiveDate.
fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect(&format!(
        "Creating a NaiveDate should not fail: year: {year:?}, month: {month:?}, day: {day:?}"
    ))
}

/// Parse a date in `dd.mm.yy` format.
fn parse_date(input: &str) -> IResult<&str, NaiveDate> {
    let (input, day) = complete::u32(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, month) = complete::u32(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, year) = complete::i32(input)?;

    Ok((input, date(2000 + year, month, day)))
}

fn parse_date_pair(input: &str) -> IResult<&str, (NaiveDate, Option<NaiveDate>)> {
    let (input, start) = parse_date(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = alt((parse_date.map(|d| Some(d)), tag("??").map(|_| None)))(input)?;

    Ok((input, (start, end)))
}

pub fn parse_many_date_pairs(input: &str) -> IResult<&str, Vec<(NaiveDate, Option<NaiveDate>)>> {
    separated_list1(newline, parse_date_pair)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_test() {
        assert_eq!(parse_date("20.11.20"), Ok(("", date(2020, 11, 20))));
        assert_eq!(parse_date("28.12.20"), Ok(("", date(2020, 12, 28))));
        assert_eq!(parse_date("13.05.21"), Ok(("", date(2021, 5, 13))));
        assert_eq!(parse_date("24.09.21"), Ok(("", date(2021, 9, 24))));
        assert_eq!(parse_date("09.05.22"), Ok(("", date(2022, 5, 9))));
        assert_eq!(parse_date("17.10.22"), Ok(("", date(2022, 10, 17))));
    }

    #[test]
    fn parse_date_pair_test() {
        assert_eq!(
            parse_date_pair("20.11.20-29.11.20"),
            Ok(("", (date(2020, 11, 20), Some(date(2020, 11, 29)))))
        );
        assert_eq!(
            parse_date_pair("30.01.21-06.02.21"),
            Ok(("", (date(2021, 1, 30), Some(date(2021, 2, 6)))))
        );
        assert_eq!(
            parse_date_pair("29.12.21-04.01.22"),
            Ok(("", (date(2021, 12, 29), Some(date(2022, 1, 4)))))
        );
        assert_eq!(
            parse_date_pair("07.03.21-??"),
            Ok(("", (date(2021, 3, 7), None)))
        );
        assert_eq!(
            parse_date_pair("24.09.21-??"),
            Ok(("", (date(2021, 9, 24), None)))
        );
        assert_eq!(
            parse_date_pair("17.10.22-??"),
            Ok(("", (date(2022, 10, 17), None)))
        );
    }
}
