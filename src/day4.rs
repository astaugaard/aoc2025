use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::BTreeSet;

use crate::day;

type Input = Vec<Vec<bool>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect_vec())
        .collect_vec())
}

fn can_remove(input: &Input, i: usize, j: usize) -> bool {
    if !input[i][j] {
        return false;
    }

    let i = i as isize;
    let j = j as isize;

    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .filter(|(di, dj)| {
        let i = di + i;
        let j = dj + j;
        if i < 0 || j < 0 || i >= input.len() as isize || j >= input[0].len() as isize {
            return true;
        }

        !input[i as usize][j as usize]
    })
    .count()
        >= 5
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        (0..input.len())
            .map(|i| {
                (0..input[0].len())
                    .filter(|j| can_remove(input, i, *j))
                    .count()
            })
            .sum::<usize>()
            .to_string(),
    )
}

fn part_b(input: &Input) -> Option<String> {
    let mut input = input.clone();

    let mut count: u32 = 0;

    let mut locations_to_check: BTreeSet<(usize, usize)> = (0..input.len())
        .flat_map(|i| (0..input[0].len()).map(move |j| (i, j)))
        .collect::<BTreeSet<_>>();

    while let Some((i, j)) = locations_to_check.pop_first() {
        if can_remove(&input, i, j) {
            input[i][j] = false;
            count += 1;
            let i = i as isize;
            let j = j as isize;

            for (di, dj) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let i = di + i;
                let j = dj + j;
                if i < 0
                    || j < 0
                    || i >= input.len() as isize
                    || j >= input[0].len() as isize
                    || !input[i as usize][j as usize]
                {
                    continue;
                }

                locations_to_check.insert((i as usize, j as usize));
            }
        }
    }

    Some(count.to_string())
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
        utils::golden("day4", &DAY, Some("13"), Some("43"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(4, &DAY, Some("1419"), Some("8739"), false);
    }
}
