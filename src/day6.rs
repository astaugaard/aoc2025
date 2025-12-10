use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = String;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input)
}

fn part_a(input: &Input) -> Option<String> {
    let mut numbers = input.lines();

    let ops = numbers
        .next_back()
        .unwrap()
        .chars()
        .filter_map(|c| if c == ' ' { None } else { Some(c == '*') })
        .collect_vec();

    let numbers = numbers
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    Some(
        ops.iter()
            .enumerate()
            .map(|(i, op)| {
                let numbers = numbers.iter().map(|nums| nums[i]);

                if *op {
                    numbers.product::<u64>()
                } else {
                    numbers.sum()
                }
            })
            .sum::<u64>()
            .to_string(),
    )
}

fn part_b(input: &Input) -> Option<String> {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let mut i = 0;

    let mut sum = 0;

    while i < input[input.len() - 1].len() {
        let mult = input[input.len() - 1][i] == '*';
        let next = input[input.len() - 1][i + 1..]
            .iter()
            .take_while(|c| **c == ' ')
            .count()
            + i
            + 1;

        let mut res: u64 = if mult { 1 } else { 0 };

        for j in i..next {
            let mut number = 0;

            let mut added_number = false;

            for d in input[0..input.len() - 1]
                .iter()
                .map(|c| c[j].to_digit(10))
                .filter(|c| c.is_some())
            {
                number *= 10;
                number += d.unwrap();

                added_number = true;
            }

            if added_number {
                if mult {
                    res *= number as u64
                } else {
                    res += number as u64
                }
            }
        }

        sum += res;

        i = next;
    }

    Some(sum.to_string())
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
        utils::golden("day6", &DAY, Some("4277556"), Some("3263827"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(
            6,
            &DAY,
            Some("6635273135233"),
            Some("12542543681221"),
            false,
        );
    }
}
