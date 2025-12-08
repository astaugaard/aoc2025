use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = Vec<(u64, u64, u64)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|a| {
            let (a, aft) = a.split_once(",").unwrap();
            let (b, c) = aft.split_once(",").unwrap();

            (
                a.parse::<u64>().unwrap(),
                b.parse::<u64>().unwrap(),
                c.parse::<u64>().unwrap(),
            )
        })
        .collect_vec())
}

fn part_a_size(input: &Input, size: usize) -> Option<String> {
    let mut connections = (0..input.len())
        .flat_map(|i| (0..input.len()).filter_map(move |j| if i < j { Some((i, j)) } else { None }))
        .collect_vec();

    connections.sort_unstable_by_key(|(a, b)| {
        let (x1, y1, z1) = input[*a];
        let (x2, y2, z2) = input[*b];

        x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)
    });

    let mut num_connections = 0;
    let mut i = 0;
    let mut union_find = (0..input.len()).collect_vec();
    let mut sizes = vec![1; input.len()];

    while i < size {
        let (from, to) = connections[i];

        if !in_same(&mut union_find, from, to) {
            num_connections += 1;
            // only works here because in_same was just called
            // so we know the one pointed to is the last element
            let l = union_find[from];
            union_find[l] = union_find[to];

            sizes[union_find[to]] += sizes[l];
            sizes[l] = 0;
        }

        i += 1;
    }

    let (_, a, aft) = sizes.select_nth_unstable(input.len() - 3);

    Some((*a * aft.iter().product::<u64>()).to_string())
}

fn in_same(union_find: &mut [usize], a: usize, b: usize) -> bool {
    find_set(union_find, a) == find_set(union_find, b)
}

fn find_set(union_find: &mut [usize], a: usize) -> usize {
    if union_find[a] == a {
        return a;
    }

    let res = find_set(union_find, union_find[a]);
    union_find[a] = res;

    res
}

fn part_a(input: &Input) -> Option<String> {
    part_a_size(input, 1000)
}

fn part_b(input: &Input) -> Option<String> {
    let mut connections = (0..input.len())
        .flat_map(|i| (0..input.len()).filter_map(move |j| if i < j { Some((i, j)) } else { None }))
        .collect_vec();

    connections.sort_unstable_by_key(|(a, b)| {
        let (x1, y1, z1) = input[*a];
        let (x2, y2, z2) = input[*b];

        x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)
    });

    let mut num_connections = 0;
    let mut i = 0;
    let mut union_find = (0..input.len()).collect_vec();
    let mut sizes = vec![1; input.len()];

    while num_connections < (input.len() - 1){
        let (from, to) = connections[i];

        if !in_same(&mut union_find, from, to) {
            num_connections += 1;
            // only works here because in_same was just called
            // so we know the one pointed to is the last element
            let l = union_find[from];
            union_find[l] = union_find[to];

            sizes[union_find[to]] += sizes[l];
            sizes[l] = 0;
        }

        i += 1;
    }

    let (f,t) = connections[i - 1];


    Some((input[f].0 * input[t].0).to_string())
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
    use crate::utils::{self, finalanswer, finalanswerrange};

    fn part_a_small(input: &Input) -> Option<String> {
        part_a_size(input, 10)
    }

    #[test]
    fn goldens() {
        utils::golden(
            "day8",
            &Lazy::new(|| day::Day {
                parser: Box::new(parser),
                part_a: Box::new(part_a_small),
                part_b: Box::new(part_b),
            }),
            Some("40"),
            Some("25272"),
            false,
        );
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(8, &DAY, Some("98696"), Some("2245203960"), false);
    }
}
