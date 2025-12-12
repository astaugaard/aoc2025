use crate::day;
use itertools::{chain, Itertools};
use once_cell::sync::Lazy;

type Input = Vec<(u64, u64)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        itertools::max(input.iter().map(|(x1, y1)| {
            itertools::max(
                input
                    .iter()
                    .map(|(x2, y2)| (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1)),
            )
            .unwrap()
        }))
        .unwrap()
        .to_string(),
    )
}

fn part_b(input: &Input) -> Option<String> {
    let mut current_max = 0;

    for (i, (x1, y1)) in input.iter().enumerate() {
        for (x2, y2) in input[0..i].iter() {
            let size = (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1);

            if size > current_max && check_valid(*x1, *y1, *x2, *y2, input) {
                current_max = size
            }
        }
    }

    Some(current_max.to_string())
}

fn check_valid(x1: u64, y1: u64, x2: u64, y2: u64, input: &Input) -> bool {
    let min_y = y1.min(y2);
    let min_x = x1.min(x2);
    let max_y = y1.max(y2);
    let max_x = x1.max(x2);

    chain!(
        input.iter().tuple_windows().map(|(x, y)| (*x, *y)),
        [(input[0], input[input.len() - 1])].into_iter()
    )
    // .par_bridge()
    .all(|((x3, y3), (x4, y4))| {
        if y3 == y4
            && min_y < y3
            && y3 < max_y
            && x3.min(x4) <= max_x
            && min_x <= x3.max(x4)
            && !((x3.min(x4) == max_x) ^ (min_x == x3.max(x4)))
        {
            return false;
        }

        if x3 == x4
            && min_x < x3
            && x3 < max_x
            && y3.min(y4) <= max_y
            && min_y <= y3.max(y4)
            && !((y3.min(y4) == max_y) ^ (min_y == y3.max(y4)))
        {
            return false;
        }
        true
    })
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
        utils::golden("day9", &DAY, Some("50"), Some("24"), false);
        utils::golden("day9-2", &DAY, None, Some("28"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(9, &DAY, Some("4746238001"), Some("1552139370"), false);
    }
}
