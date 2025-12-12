use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Input {
    // shapes: Vec<Vec<Vec<bool>>>,
    fields: Vec<((u32, u32), Vec<u32>)>,
}

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let fields = input
        .lines()
        .map(|f| {
            let (size, amounts) = f.split_once(": ").unwrap();

            let (w, h) = size.split_once("x").unwrap();

            let amounts = amounts
                .split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec();

            (
                (w.parse::<u32>().unwrap(), h.parse::<u32>().unwrap()),
                amounts,
            )
        })
        .collect_vec();

    Ok(Input { fields })
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .fields
            .iter()
            .filter(|field| field_works(field))
            .count()
            .to_string(),
    )
}

fn field_works(((w, h), amounts): &((u32, u32), Vec<u32>)) -> bool {
    let max = w * h;

    let mut space_used = 0;

    if amounts[0] % 2 == 0 {
        space_used += amounts[0] * 6;
    } else {
        space_used += amounts[0] / 2 * 12 + 9;
    }

    if amounts[0] % 2 == 0 {
        space_used += amounts[0] / 2 * 15;
    } else {
        space_used += amounts[0] / 2 * 15 + 9;
    }

    space_used += amounts[2] * 6 + 6;

    space_used += amounts[3] * 9;

    space_used += amounts[4] * 9;

    space_used += amounts[5] * 9;

    space_used <= max
}

fn part_b(_input: &Input) -> Option<String> {
    None
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
    fn test_finalanswer() {
        finalanswer(12, &DAY, Some("519"), None, false);
    }
}
