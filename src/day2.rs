use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = Vec<(String, String)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| {
            let (b, a) = s.split_once("-").unwrap();
            (b.to_string(), a.to_string())
        })
        .collect::<Vec<_>>())
}

fn find_halfs(a: &str) -> (usize, usize) {
    let (a, b) = a.split_at(a.len() / 2);
    (
        str::parse::<usize>(a).unwrap(),
        str::parse::<usize>(b).unwrap(),
    )
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .map(|(from, to)| {
                let ((from1, from2), (to1, to2)) = if from.len() > to.len() {
                    return 0;
                } else if to.len() > from.len() {
                    if to.len() - from.len() > 1 {
                        panic!("I haven't implemented this case")
                    }

                    if to.len() % 2 == 0 {
                        ((10_usize.pow((to.len() / 2 - 1) as u32), 0), find_halfs(to))
                    } else {
                        let max_of_lenth = 10_usize.pow((from.len() / 2) as u32) - 1;
                        (find_halfs(from), (max_of_lenth, max_of_lenth))
                    }
                } else if to.len() % 2 == 0 {
                    (find_halfs(from), find_halfs(to))
                } else {
                    return 0;
                };

                let f = if from1 < from2 { from1 + 1 } else { from1 };
                let t = if to2 < to1 { to1 - 1 } else { to1 };

                if f > t {
                    return 0;
                }

                let sum = (f + t) * (t - f + 1) / 2;

                let mult = 10_usize.pow(from.len().div_ceil(2) as u32);

                sum * mult + sum
            })
            .sum::<usize>()
            .to_string(),
    )
}

fn split_sized(s: &str, size: usize) -> Vec<usize> {
    s.chars()
        .chunks(size)
        .into_iter()
        .map(|chunk| {
            let mut sum: usize = 0;

            for c in chunk {
                sum *= 10;
                sum += c.to_digit(10).unwrap() as usize;
            }

            sum
        })
        .collect::<Vec<_>>()
}

fn compute_repititions_sized_sum_from_bounds(froms: &[usize], tos: &[usize], size: usize) -> usize {
    let mut cur_f = froms[froms.len() - 1];

    for i in (0..(froms.len() - 1)).rev() {
        cur_f = if froms[i] < cur_f {
            froms[i] + 1
        } else {
            froms[i]
        }
    }

    let mut cur_t = tos[tos.len() - 1];

    for i in (0..(tos.len() - 1)).rev() {
        cur_t = if cur_t < tos[i] { tos[i] - 1 } else { tos[i] }
    }

    dbg!(cur_f);
    dbg!(cur_t);

    if cur_f > cur_t {
        return 0;
    }

    let sum = (cur_f + cur_t) * (cur_t - cur_f + 1) / 2;

    let mut mult = 1;

    for _ in 0..(froms.len() - 1) {
        mult *= 10_usize.pow(size as u32);
        mult += 1;
    }

    dbg!(sum) * dbg!(mult)
}

fn compute_repititions_sized_sum(from: &str, to: &str, size: usize) -> (usize, usize) {
    if to.len() - from.len() > 1 {
        panic!("case not considered")
    }

    dbg!(size);

    let v = if from.len() > to.len() {
        return (0, 0);
    } else if to.len() > from.len() {
        if to.len() - from.len() > 1 {
            panic!("I haven't implemented this case")
        }

        let mut out = vec![];

        if to.len().is_multiple_of(size) {
            let mut lower = vec![10_usize.pow((size - 1) as u32)];

            while lower.len() < to.len() / size {
                lower.push(0);
            }

            out.push((lower, split_sized(to, size)))
        }

        if from.len().is_multiple_of(size) && from.len() != size {
            let mut max_of_lenth = vec![];

            while max_of_lenth.len() < from.len() / size {
                max_of_lenth.push(10_usize.pow(size as u32) - 1)
            }

            out.push((split_sized(from, size), max_of_lenth))
        }

        out
    } else if to.len().is_multiple_of(size) {
        vec![(split_sized(from, size), split_sized(to, size))]
    } else {
        return (0, 0);
    };

    let mut res = (0, 0);

    for (froms, tos) in v {
        let a = compute_repititions_sized_sum_from_bounds(&froms, &tos, size);
        if froms.len() * size == from.len() {
            res.0 = a
        } else {
            res.1 = a
        }
    }

    res
}

fn part_b(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .map(|(from, to)| part_b_instance_fast(from, to))
            .sum::<usize>()
            .to_string(),
    )
}

fn part_b_instance_fast(from: &str, to: &str) -> usize {
    if from.len() > to.len() {
        return 0;
    }

    let mut amounts = (1..(from.len().max(to.len())))
        .map(|len| {
            let from_works = from.len().is_multiple_of(len) && from.len() != len;
            let to_works = to.len().is_multiple_of(len) && to.len() != len;

            if from_works || to_works {
                compute_repititions_sized_sum(from, to, len)
            } else {
                (0, 0)
            }
        })
        .collect_vec();

    for i in 0..amounts.len() {
        let i = i + 1;
        if amounts[i - 1] == (0, 0) {
            continue;
        }
        for j in 1..i {
            if i.is_multiple_of(j) {
                if amounts[i - 1].0 != 0 {
                    amounts[i - 1].0 -= amounts[j - 1].0;
                }
                if amounts[i - 1].1 != 0 {
                    amounts[i - 1].1 -= amounts[j - 1].1;
                }
            }
        }
    }

    amounts.iter().map(|(a, b)| a + b).sum()
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
    use crate::utils::{self};
    use fancy_regex::Regex;
    use proptest::prelude::*;

    #[test]
    fn goldens() {
        utils::golden("day2", &DAY, Some("1227775554"), Some("4174379265"), false);
    }

    fn part_b_instance(from: &str, to: &str, re: &Regex) -> usize {
        let from: usize = from.parse::<usize>().unwrap();
        let to: usize = to.parse::<usize>().unwrap();

        (from..=to)
            .filter(|i| re.is_match(&i.to_string()).unwrap())
            .sum::<usize>()
    }

    proptest! {
        #[test]
        fn two_equal(
          m in 1..1000u64,
          n in 1..200_u64,
        ) {
            let a = m.to_string();
            let b = (m + n).to_string();

            let re = Regex::new(r"^(\d+)\1+$").unwrap();

            prop_assume!((a.len() as i64- b.len() as i64).abs() <= 1);

            assert_eq!(part_b_instance_fast(&a, &b), part_b_instance(&a,&b,&re))
        }
    }

    #[test]
    fn rep12() {
        assert_eq!(part_b_instance_fast("1", "11"), 11)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(2, &DAY, Some("41294979841"), Some("66500947346"), false);
    }
}
