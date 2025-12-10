use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::BTreeSet;

type Input = Vec<Vec<bool>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| line.chars().map(|c| c == '^').collect_vec())
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let mut locs = BTreeSet::new();

    locs.insert(input[0].len() / 2);

    let mut splits = 0;

    for row in input {
        let mut new_set = BTreeSet::new();

        for l in &locs {
            if row[*l] {
                splits += 1;
                new_set.insert(l - 1);
                new_set.insert(l + 1);
            } else {
                new_set.insert(*l);
            }
        }

        locs = new_set;
    }

    Some(splits.to_string())
}

fn part_b(input: &Input) -> Option<String> {
    let mut ways: Vec<Vec<usize>> = (0..input.len())
        .map(|_| vec![0; input[0].len()])
        .collect_vec();

    for i in &mut ways[input.len() - 1] {
        *i = 1;
    }

    for i in (0..(input.len() - 1)).rev() {
        for j in 0..input[0].len() {
            ways[i][j] = if input[i + 1][j] {
                ways[i + 1][j - 1] + ways[i + 1][j + 1]
            } else {
                ways[i + 1][j]
            };
        }
    }

    Some(ways[0][(input[0].len()) / 2].to_string())
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
        utils::golden("day7", &DAY, Some("21"), Some("40"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(7, &DAY, Some("1560"), Some("25592971184998"), false);
    }
}
