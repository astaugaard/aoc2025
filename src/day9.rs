use crate::day;
use itertools::{chain, Itertools};
use once_cell::sync::Lazy;
use std::collections::HashMap;

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

    let mut x_cordinates = input.iter().map(|a| a.0).collect_vec();
    let mut y_cordinates = input.iter().map(|a| a.1).collect_vec();

    x_cordinates.sort_unstable();
    x_cordinates.dedup();
    y_cordinates.sort_unstable();
    y_cordinates.dedup();

    let map_x = x_cordinates
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect::<HashMap<u64, usize>>();

    let map_y = y_cordinates
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect::<HashMap<u64, usize>>();

    let mut strunk_border = vec![vec![false; x_cordinates.len() + 2]; y_cordinates.len() + 2];

    for ((x1, y1), (x2, y2)) in chain!(
        input.iter().tuple_windows().map(|(x, y)| (*x, *y)),
        [(input[0], input[input.len() - 1])].into_iter()
    ) {
        if x1 == x2 {
            let x = map_x[&x1];
            for row in &mut strunk_border[map_y[&y1.min(y2)]..=map_y[&y1.max(y2)]] {
                row[x] = true;
            }
        }

        if y1 == y2 {
            let row = &mut strunk_border[map_y[&y1]];

            for loc in &mut row[map_x[&x1.min(x2)]..=map_x[&x1.max(x2)]] {
                *loc = true;
            }
        }
    }

    let mut inside = vec![vec![true; x_cordinates.len() + 2]; y_cordinates.len() + 2];

    flood_fill(&mut inside, &strunk_border);

    for (i, (x1, y1)) in input.iter().enumerate() {
        for (x2, y2) in input[0..i].iter() {
            let size = (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1);

            if size > current_max && check_valid_shrunk(*x1, *y1, *x2, *y2, &map_x, &map_y, &inside)
            {
                current_max = size
            }
        }
    }

    Some(current_max.to_string())
}

fn check_valid_shrunk(
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
    map_x: &HashMap<u64, usize>,
    map_y: &HashMap<u64, usize>,
    shrunk: &[Vec<bool>],
) -> bool {
    let x1 = map_x[&x1];
    let x2 = map_x[&x2];
    let y1 = map_y[&y1];
    let y2 = map_y[&y2];

    (x1..=x2).all(|x| (y1..=y2).all(|y| shrunk[y][x]))
}

fn flood_fill(inside: &mut [Vec<bool>], strunk_border: &[Vec<bool>]) {
    flood_fill_rec(0, 0, inside, strunk_border);
}

fn flood_fill_rec(x: i32, y: i32, inside: &mut [Vec<bool>], strunk_border: &[Vec<bool>]) {
    if x < 0 || y < 0 || x as usize >= inside[0].len() || y as usize >= inside.len() {
        return;
    }

    if !inside[y as usize][x as usize] || strunk_border[y as usize][x as usize] {
        return;
    }

    inside[y as usize][x as usize] = false;

    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let x = x + dx;
        let y = y + dy;

        flood_fill_rec(x, y, inside, strunk_border);
    }
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
