use chrono::{Duration, NaiveDate};
use statrs::distribution::{Discrete, Poisson};

pub struct DateDiffDistribution {
    distrib: Poisson,
    most_recent: NaiveDate,
    cycles_to_predict: u32,
}

impl DateDiffDistribution {
    pub fn new(past_data: &[NaiveDate], cycles_to_predict: u32) -> Self {
        let rate = past_data
            .array_windows()
            .map(|[a, b]| b.signed_duration_since(*a).num_days())
            .sum::<i64>()
            / past_data.len() as i64;

        Self {
            distrib: Poisson::new(rate as f64).expect("The rate should be > 0"),
            most_recent: *past_data.last().unwrap(),
            cycles_to_predict,
        }
    }

    /// Find the probability of a period starting on the given date.
    pub fn find_probability(&self, date: NaiveDate) -> f64 {
        (0..self.cycles_to_predict)
            .into_iter()
            .map(|cycles_after_end| {
                let days = (date
                    - self.most_recent
                    - Duration::days((cycles_after_end * self.distrib.lambda() as u32).into()))
                .num_days();
                let days: u64 = match days.try_into() {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                self.distrib.pmf(days)
            })
            .sum()
    }
}
