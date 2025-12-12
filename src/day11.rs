use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;

type Input = HashMap<String, Vec<String>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(": ").unwrap();

            let to = to.split(' ').map(|to| to.to_string()).collect_vec();

            (from.to_string(), to)
        })
        .collect::<HashMap<_, _>>())
}

fn part_a(input: &Input) -> Option<String> {
    let mut res: Vec<&String> = vec![];

    let mut added = HashMap::new();

    added.insert("out", 0);

    let out = "out".to_string();

    res.push(&out);

    let you = "you".to_string();

    dfs_topsort(input, &you, &mut added, &mut res);

    let mut ways = vec![0; res.len()];

    ways[0] = 1;

    let mut a = res.into_iter();

    a.next().unwrap();

    for (i, r) in a.enumerate() {
        let i = i + 1;

        ways[i] = input[r].iter().map(|a| ways[added[a.as_str()]]).sum();
    }

    Some(ways[ways.len() - 1].to_string())
}

fn dfs_topsort<'a>(
    input: &'a Input,
    at: &'a String,
    added: &mut HashMap<&'a str, usize>,
    res: &mut Vec<&'a String>,
) {
    if added.contains_key(at.as_str()) {
        return;
    }

    if let Some(i) = input.get(at) {
        for i in i {
            dfs_topsort(input, i, added, res);
        }
    }

    added.insert(at, res.len());

    res.push(at);
}

fn part_b(input: &Input) -> Option<String> {
    let mut res: Vec<&String> = vec![];

    let mut added = HashMap::new();

    added.insert("out", 0);

    let out = "out".to_string();

    res.push(&out);

    let svr = "svr".to_string();

    dfs_topsort(input, &svr, &mut added, &mut res);

    let mut ways = [vec![0u64; res.len()], vec![0; res.len()], vec![0; res.len()]];

    ways[0][0] = 1;

    let mut a = res.into_iter();

    a.next().unwrap();

    for (i, r) in a.enumerate() {
        let i = i + 1;

        if r == "fft" || r.as_str() == "dac" {
            for j in 1..3 {
                ways[j][i] = input[r]
                    .iter()
                    .map(|a| ways[j - 1][added[a.as_str()]])
                    .sum();
            }
        } else {
            for w in &mut ways {
                w[i] = input[r]
                    .iter()
                    .map(|a| w[added[a.as_str()]])
                    .sum();
            }
        }
    }

    Some(ways[2][ways[2].len() - 1].to_string())
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
        utils::golden("day11", &DAY, Some("5"), None, false);
    }

    #[test]
    fn goldens_2() {
        utils::golden("day11-2", &DAY, None, Some("2"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(11, &DAY, Some("497"), Some("358564784931864"), false);
    }
}
