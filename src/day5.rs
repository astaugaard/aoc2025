use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    ranges: Vec<(u64, u64)>,
    numbers: Vec<u64>,
}

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let (ranges, numbers) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|range| {
            let (lower, upper) = range.split_once("-").unwrap();

            (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap())
        })
        .collect_vec();

    let numbers = numbers
        .lines()
        .map(|number| number.parse::<u64>().unwrap())
        .collect_vec();

    Ok(Input { ranges, numbers })
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .numbers
            .iter()
            .filter(|number| {
                input
                    .ranges
                    .iter()
                    .any(|(lower, upper)| lower <= number && **number <= *upper)
            })
            .count()
            .to_string(),
    )
}

fn part_b(input: &Input) -> Option<String> {
    let mut ranges = input.ranges.clone();

    for i in 1..ranges.len() {
        for j in 0..i {
            let (lower_bound_old, upper_bound_old) = ranges[j];
            let (lower_bound_new, upper_bound_new) = &mut ranges[i];

            if lower_bound_old <= *lower_bound_new && *lower_bound_new <= upper_bound_old {
                *lower_bound_new = upper_bound_old + 1;
            }

            if lower_bound_old <= *upper_bound_new && *upper_bound_new <= upper_bound_old {
                *upper_bound_new = lower_bound_old - 1
            }
        }
    }

    for i in (0..(ranges.len()-1)).rev() {
        for j in ((i+1)..(ranges.len())).rev() {
            let (lower_bound_old, upper_bound_old) = ranges[j];
            let (lower_bound_new, upper_bound_new) = &mut ranges[i];

            if lower_bound_old <= *lower_bound_new && *lower_bound_new <= upper_bound_old {
                *lower_bound_new = upper_bound_old + 1;
            }

            if lower_bound_old <= *upper_bound_new && *upper_bound_new <= upper_bound_old {
                *upper_bound_new = lower_bound_old - 1
            }
        }
    }

    Some(
        ranges
            .iter()
            .map(|(lower, upper)| if upper >= lower { upper - lower + 1 } else { 0 })
            .sum::<u64>()
            .to_string(),
    )
}

pub static DAY: Lazy<day::Day<Input>> = Lazy::new(|| day::Day {
    // do not touch
    parser: Box::new(parser),
    part_a: Box::new(part_a),
    part_b: Box::new(part_b),
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{self, finalanswer};

    #[test]
    fn goldens() {
        utils::golden("day5", &DAY, Some("3"), Some("14"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(5, &DAY, Some("712"), Some("332998283036769"), false);
    }
}
