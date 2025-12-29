use crate::day;
use itertools::Itertools;
use num_rational::Rational32;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Machine {
    signal_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

type Input = Vec<Machine>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input.lines().map(parse_machine).collect_vec())
}

fn parse_machine(m: &str) -> Machine {
    let mut buttons = m.split(' ');
    let joltages = buttons.next_back().unwrap();
    let signal_lights = buttons.next().unwrap();
    // now buttons only holds the buttons

    let mut signal_lights = signal_lights.chars();
    signal_lights.next();

    let signal_lights = signal_lights
        .take_while(|a| *a != ']')
        .map(|a| a == '#')
        .collect_vec();

    let joltages = joltages[1..joltages.len() - 1]
        .split(',')
        .map(|a| a.parse::<u32>().unwrap())
        .collect_vec();

    let buttons = buttons
        .map(|b| {
            b[1..b.len() - 1]
                .split(',')
                .map(|a| a.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    Machine {
        signal_lights,
        buttons,
        joltages,
    }
}

fn gauss_binary(input: &mut Vec<Vec<bool>>) {
    for i in 0..(input.len() - 1) {
        let mut min = i;
        let mut min_value = 99999999999; // random number that I know we won't have a larger number in the input

        (min..input.len()).for_each(|j| {
            let num = input[j].iter().take_while(|a| !**a).count();

            if num < min_value {
                min_value = num;
                min = j;
            }
        });

        input.swap(i, min);

        if min_value >= input[0].len() {
            continue;
        }

        for j in i + 1..input.len() {
            if input[j][min_value] {
                for k in min_value..input[j].len() {
                    input[j][k] ^= input[i][k];
                }
            }
        }

        for j in 0..i {
            if input[j][min_value] {
                for k in min_value..input[j].len() {
                    input[j][k] ^= input[i][k];
                }
            }
        }
    }
}

fn part_a(input: &Input) -> Option<String> {
    Some(input.iter().map(solve_a).sum::<usize>().to_string())
}

fn brute_force_minimal_solution(a: Vec<Vec<bool>>) -> usize {
    let mut upper_bound = a.iter().filter(|r| *r.last().unwrap()).count();
    let mut assignments = Vec::with_capacity(a[0].len() - 1);

    brute_force_minimal_solution_go(&a, &mut assignments, 0, &mut upper_bound);

    upper_bound
}

fn brute_force_minimal_solution_go(
    a: &Vec<Vec<bool>>,
    assignments: &mut Vec<bool>,
    on: usize,
    upper_bound: &mut usize,
) {
    if on >= *upper_bound {
        return;
    }

    if assignments.len() == a[0].len() - 1 {
        let score = diff_solution(a, assignments) + on;

        *upper_bound = score.min(*upper_bound);

        return;
    }

    assignments.push(true);

    brute_force_minimal_solution_go(a, assignments, on + 1, upper_bound);

    assignments.pop();

    assignments.push(false);

    brute_force_minimal_solution_go(a, assignments, on, upper_bound);

    assignments.pop();
}

fn diff_solution(a: &[Vec<bool>], assignments: &Vec<bool>) -> usize {
    a.iter()
        .filter(|r| {
            r[0..(r.len() - 1)]
                .iter()
                .zip(assignments)
                .fold(false, |r, (c, var)| r ^ (*c && *var))
                != *r.last().unwrap()
        })
        .count()
}

fn solve_a(a: &Machine) -> usize {
    let mut expressions: Vec<Vec<bool>> =
        vec![vec![false; a.buttons.len() + 1]; a.signal_lights.len()];

    for (e, l) in expressions.iter_mut().zip(&a.signal_lights) {
        let last = e.len() - 1;

        e[last] = *l;
    }

    for (i, button) in a.buttons.iter().enumerate() {
        for flip in button {
            expressions[*flip][i] = true;
        }
    }

    gauss_binary(&mut expressions);

    let mut leading_column = vec![false; a.buttons.len() + 1];

    for e in &expressions {
        for (i, a) in e.iter().enumerate() {
            if *a {
                leading_column[i] = true;
                break;
            }
        }
    }

    for e in &mut expressions {
        let mut i = 0;

        e.retain(|_| {
            let res = leading_column[i];

            i += 1;
            !res
        })
    }

    brute_force_minimal_solution(expressions)
}

fn gauss_regular(input: &mut [Vec<Rational32>]) {
    for i in 0..input.len() {
        let mut min = i;
        let mut min_value = 99999999999; // random number that I know we won't have a larger number in the input

        (min..input.len()).for_each(|j| {
            let num = input[j]
                .iter()
                .take_while(|a| **a == Rational32::from_integer(0))
                .count();

            if num < min_value {
                min_value = num;
                min = j;
            }
        });

        input.swap(i, min);

        if min_value >= input[0].len() {
            continue;
        }

        let div = input[i][min_value];

        for k in &mut input[i] {
            *k /= div;
        }

        for j in i + 1..input.len() {
            if input[j][min_value] != Rational32::from_integer(0) {
                let mult = input[j][min_value];
                for k in min_value..input[j].len() {
                    let sub = input[i][k] * mult;
                    input[j][k] -= sub;
                }
            }
        }

        for j in 0..i {
            if input[j][min_value] != Rational32::from_integer(0) {
                let mult = input[j][min_value];
                for k in min_value..input[j].len() {
                    let sub = input[i][k] * mult;
                    input[j][k] -= sub;
                }
            }
        }
    }
}

fn solve_b_matrix(a: &Machine) -> usize {
    let mut expressions: Vec<Vec<Rational32>> =
        vec![vec![Rational32::from_integer(0); a.buttons.len() + 1]; a.signal_lights.len()];

    for (e, l) in expressions.iter_mut().zip(&a.joltages) {
        let last = e.len() - 1;

        e[last] = Rational32::from_integer(*l as i32);
    }

    for (i, button) in a.buttons.iter().enumerate() {
        for flip in button {
            expressions[*flip][i] = Rational32::from_integer(1);
        }
    }

    gauss_regular(&mut expressions);

    let mut leading_column = vec![false; a.buttons.len() + 1];

    for e in &expressions {
        for (i, a) in e.iter().enumerate() {
            if *a != Rational32::from_integer(0) {
                leading_column[i] = true;
                break;
            }
        }
    }

    for e in &mut expressions {
        let mut i = 0;

        e.retain(|_| {
            let res = leading_column[i];

            i += 1;
            !res
        })
    }

    brute_force_minimal_solution_b(expressions)
}

fn brute_force_minimal_solution_b(a: Vec<Vec<Rational32>>) -> usize {
    let current_sums = vec![Rational32::from_integer(0); a.len()];

    let max_min: Vec<(Vec<Rational32>, Vec<Rational32>)> = a
        .iter()
        .map(|a| {
            let mut maxes = a[0..a.len() - 1]
                .iter()
                .rev()
                .scan(Rational32::from_integer(1), |a, b| {
                    *a = (*a).max(*b);
                    Some(*a)
                })
                .collect_vec();

            let mut mins = a[0..a.len() - 1]
                .iter()
                .rev()
                .scan(Rational32::from_integer(0), |a, b| {
                    *a = (*a).min(*b);
                    Some(*a)
                })
                .collect_vec();

            maxes.reverse();
            mins.reverse();

            (maxes, mins)
        })
        .collect_vec();

    let mut upper_bound = *a
        .iter()
        .zip(&max_min)
        .map(|(r, (_, min))| {
            let res = *r.last().unwrap();
            if res < Rational32::from_integer(0) {
                (res / min[0]).ceil()
            } else {
                res
            }
        })
        .sum::<Rational32>()
        .floor()
        .numer() as usize;

    brute_force_minimal_solution_go_b(&a, 0, 0, &current_sums, &mut upper_bound, &max_min);

    upper_bound
}

fn brute_force_minimal_solution_go_b(
    a: &Vec<Vec<Rational32>>,
    mut on: usize,
    var: usize,
    current_sums: &Vec<Rational32>,
    upper_bound: &mut usize,
    max_min: &Vec<(Vec<Rational32>, Vec<Rational32>)>,
) {
    if on >= *upper_bound {
        return;
    }

    if var == a[0].len() - 1 {
        for (s, a) in current_sums.iter().zip(a) {
            if *s > *a.last().unwrap() {
                return;
            }
        }

        if let Some(diff) = diff_solution_b(a, current_sums) {
            let score = diff + on;
            *upper_bound = score.min(*upper_bound);
        }

        return;
    }

    let mut current_sums = current_sums.clone();

    loop {
        brute_force_minimal_solution_go_b(a, on, var + 1, &current_sums, upper_bound, max_min);

        on += 1;

        if on >= *upper_bound {
            return;
        }

        for ((s, a), (max, min)) in current_sums.iter_mut().zip(a).zip(max_min) {
            *s += a[var];

            if a[var] == Rational32::from_integer(0) {
                continue;
            }

            let diff = (*a.last().unwrap()) - *s;

            if diff < Rational32::from_integer(0)
                && Rational32::from_integer((*upper_bound - on) as i32) * min[var] > diff
            {
                return;
            }

            if diff > Rational32::from_integer(0)
                && Rational32::from_integer((*upper_bound - on) as i32) * max[var] < diff
            {
                return;
            }
        }
    }
}

fn diff_solution_b(a: &[Vec<Rational32>], current_sums: &Vec<Rational32>) -> Option<usize> {
    let diff = a
        .iter()
        .zip(current_sums)
        .map(|(r, c)| {
            let res = r.last().unwrap() - c;
            if res < Rational32::from_integer(0) || *res.denom() != 1 {
                None
            } else {
                Some(res)
            }
        })
        .reduce(|a, b| match (a, b) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        })??;

    Some(*(diff.floor().numer()) as usize)
}

fn part_b(input: &Input) -> Option<String> {
    Some(
        input
            .par_iter()
            .map(|r| {
                let new = solve_b_matrix(r);

                new
            })
            .sum::<usize>()
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
    use crate::utils::{self, finalanswer};

    #[test]
    fn goldens() {
        utils::golden("day10", &DAY, Some("7"), Some("33"), false);
    }

    #[test]
    fn test_finalanswer() {
        finalanswer(10, &DAY, Some("411"), Some("16063"), false);
    }
}
