use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::day;

type Input = Vec<Vec<u8>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec())
}

fn line_a(line: &[u8]) -> u32 {
    let mut digit1 = line[line.len() - 2];
    let mut digit2 = line[line.len() - 1];

    for i in (0..line.len() - 2).rev() {
        if line[i] >= digit1 {
            digit2 = digit1.max(digit2);
            digit1 = line[i]
        }
    }

    (digit1 * 10 + digit2) as u32
}

fn part_a(input: &Input) -> Option<String> {
    Some(input.iter().map(|a| line_a(a)).sum::<u32>().to_string())
}

fn line_b(line: &[u8]) -> u64 {
    let mut memo: [Vec<u64>; 12] = core::array::from_fn(|_| line.iter().map(|_| 0).collect_vec());

    let mut max = 0;

    for i in (0..line.len()).rev() {
        max = line[i].max(max);
        memo[0][i] = max as u64;
    }

    for i in 2..=12 {
        let simple_best = memo[i - 2][line.len() - i + 1]
            + line[line.len() - i] as u64 * 10_u64.pow(i as u32 - 1);

        memo[i - 1][line.len() - i] = simple_best;

        for j in (0..=(line.len() - i - 1)).rev() {
            let potential_best = memo[i - 2][j + 1] + (line[j] as u64 * 10_u64.pow(i as u32 - 1));

            memo[i - 1][j] = memo[i - 1][j + 1].max(potential_best);
        }
    }

    memo[11][0]
}

fn part_b(input: &Input) -> Option<String> {
    Some(input.iter().map(|a| line_b(a)).sum::<u64>().to_string())
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
        utils::golden("day3", &DAY, Some("357"), Some("3121910778619"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(3, &DAY, Some("17330"), Some("171518260283767"), false);
        // finalanswerrange(2, &DAY, Some(18734850051), None, vec![], Some(66500947335), None, vec![]);
    }
}
