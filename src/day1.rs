use once_cell::sync::Lazy;

use crate::day;

type Input = Vec<(bool, i16)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            Ok::<(bool, i16), String>((
                dir == "L",
                str::parse::<i16>(num).map_err(|a| a.to_string())?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .scan(50, |acc, (left, num)| {
                *acc += if *left { *num } else { -num };

                *acc %= 100;

                Some(*acc)
            })
            .map(|a| if a == 0 { 1 } else { 0 })
            .sum::<i16>()
            .to_string(),
    )
}

fn part_b(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .scan(50, |acc, (left, num)| {
                let zero_before = *acc == 0;

                *acc += if *left { -num } else { *num };

                let mut res = {
                    acc.div_euclid(100).abs()
                };

                *acc = acc.rem_euclid(100);

                if zero_before && *left && *acc != 0 {
                    res -= 1
                }

                if !zero_before && *acc == 0 && *left {
                    res += 1
                }

                dbg!(acc);

                Some(dbg!(res))
            })
            .sum::<i16>()
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
    use crate::utils;

    #[test]
    fn goldens() {
        utils::golden("day1", &DAY, Some("3"), Some("6"), false);
        utils::golden("day1-1000", &DAY, None, Some("10"), false);
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(1, &DAY, Some("1150"), Some("6738"), false);
    }
}
