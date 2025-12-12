use crate::day;
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};
use itertools::Itertools;
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
        .take_while_inclusive(|a| *a != ']')
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

fn part_a(input: &Input) -> Option<String> {
    Some(input.iter().map(solve_a).sum::<usize>().to_string())
}

fn solve_a(a: &Machine) -> usize {
    let mut variables = variables! {};

    let buttons = variables.add_vector(variable().binary(), a.buttons.len());

    let helper_scalers = variables.add_vector(variable().integer(), a.signal_lights.len());

    let mut expressions = vec![Expression::with_capacity(0); a.signal_lights.len()];

    for (i, button) in a.buttons.iter().enumerate() {
        for flip in button {
            expressions[*flip] += buttons[i];
        }
    }

    let minimization: Expression = buttons.into_iter().sum1().unwrap();

    let mut system = variables.minimise(&minimization).using(default_solver);

    system.set_parameter("log", "0");

    let solution = system
        .with_all(
            expressions
                .into_iter()
                .zip(a.signal_lights.iter().cloned())
                .enumerate()
                .map(|(i, (ex, s))| ex.eq(2 * helper_scalers[i] + if s { 1 } else { 0 })),
        )
        .solve()
        .unwrap();

    solution.eval(&minimization) as usize
}

fn solve_b(a: &Machine) -> usize {
    let mut variables = variables! {};

    let buttons = variables.add_vector(variable().integer().min(0), a.buttons.len());

    let mut expressions = vec![Expression::with_capacity(0); a.joltages.len()];

    for (i, button) in a.buttons.iter().enumerate() {
        for flip in button {
            expressions[*flip] += buttons[i];
        }
    }

    let minimization: Expression = buttons.into_iter().sum1().unwrap();

    let mut system = variables.minimise(&minimization).using(default_solver);

    system.set_parameter("log", "0");

    let solution = system
        .with_all(
            expressions
                .into_iter()
                .enumerate()
                .map(|(i, ex)| ex.eq(a.joltages[i])),
        )
        .solve()
        .unwrap();

    solution.eval(&minimization) as usize
}

fn part_b(input: &Input) -> Option<String> {
    Some(input.iter().map(solve_b).sum::<usize>().to_string())
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
