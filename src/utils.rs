use std::fs;

use once_cell::sync::Lazy;

use crate::day::Day;

pub fn golden<'a, Input>(
    file: &'a str,
    parent: &'a Lazy<Day<Input>>,
    expected_a: Option<&'a str>,
    expected_b: Option<&'a str>,
    verbose: bool,
) {
    let input = match fs::read_to_string(format!("goldens/{}", file)) {
        Ok(a) => a,
        Err(_) => panic!("golden {} failed: could not open file", file),
    };

    let parent = Lazy::force(parent);

    let input = match (*parent.parser)(input, verbose) {
        Ok(a) => a,
        Err(err) => panic!("golden {} failed to parse: {}", file, err),
    };

    let part_a = (*parent.part_a)(&input);
    let part_b = (*parent.part_b)(&input);

    if let Some(_a) = expected_a {
        if expected_a != part_a.as_deref() {
            panic!("golden {} expected {:?} got {:?}", file, expected_a, part_a);
        }
    }

    if let Some(_a) = expected_b {
        if expected_b != part_b.as_deref() {
            panic!("golden {} expected {:?} got {:?}", file, expected_b, part_b);
        }
    }
}

pub fn finalanswer<'a, Input>(
    daynum: usize,
    parent: &'a Lazy<Day<Input>>,
    expected_a: Option<&'a str>,
    expected_b: Option<&'a str>,
    verbose: bool,
) {
    let input = match fs::read_to_string(format!("inputs/day{}", daynum)) {
        Ok(a) => a,
        Err(_) => panic!(
            "regression test for day: {} failed: could not open file",
            daynum
        ),
    };

    let parent = Lazy::force(parent);

    let input = match (*parent.parser)(input, verbose) {
        Ok(a) => a,
        Err(err) => panic!(
            "regression test for day: {} failed to parse: {}",
            daynum, err
        ),
    };

    let part_a = (*parent.part_a)(&input);
    let part_b = (*parent.part_b)(&input);

    if let Some(_a) = expected_a {
        if expected_a != part_a.as_deref() {
            panic!(
                "regression test for day: {} expected {:?} got {:?}",
                daynum, expected_a, part_a
            );
        }
    }

    if let Some(_a) = expected_b {
        if expected_b != part_b.as_deref() {
            panic!(
                "regression test for day: {} expected {:?} got {:?}",
                daynum, expected_b, part_b
            );
        }
    }
}

pub fn finalanswerrange<'a, Input>(
    daynum: usize,
    parent: &'a Lazy<Day<Input>>,
    lower_bound_a: Option<i64>,
    upper_bound_a: Option<i64>,
    not_equal_a: Vec<i64>,
    lower_bound_b: Option<i64>,
    upper_bound_b: Option<i64>,
    not_equal_b: Vec<i64>,
) {
    let input = match fs::read_to_string(format!("inputs/day{}", daynum)) {
        Ok(a) => a,
        Err(_) => panic!(
            "regression test for day: {} failed: could not open file",
            daynum
        ),
    };

    let parent = Lazy::force(parent);

    let input = match (*parent.parser)(input, false) {
        Ok(a) => a,
        Err(err) => panic!(
            "regression test for day: {} failed to parse: {}",
            daynum, err
        ),
    };

    let part_a = (*parent.part_a)(&input);
    let part_b = (*parent.part_b)(&input);

    if let Some(l) = lower_bound_a {
        if l >= str::parse::<i64>(&(part_a.clone()).unwrap()).unwrap() {
            panic!("day bounds failed: {}, {l} >= {}", daynum, part_a.unwrap());
        }
    }

    if let Some(l) = upper_bound_a {
        if l <= str::parse::<i64>(&(part_a.clone()).unwrap()).unwrap() {
            panic!("day bounds failed: {}, {l} <= {}", daynum, part_a.unwrap());
        }
    }

    for l in not_equal_a {
        if l != str::parse::<i64>(&(part_a.clone()).unwrap()).unwrap() {
            panic!("day bounds failed: {}, {l} != {}", daynum, part_a.unwrap());
        }
    }
    if let Some(l) = lower_bound_b {
        if l >= str::parse::<i64>(&(part_b.clone()).unwrap()).unwrap() {
            panic!("day bounds failed: {}, {l} >= {}", daynum, part_b.unwrap());
        }
    }

    if let Some(l) = upper_bound_b {
        if l <= str::parse::<i64>(&(part_b.clone()).unwrap()).unwrap() {
            panic!("day bounds failed: {}, {l} <= {}", daynum, part_b.unwrap());
        }
    }

    for l in not_equal_b {
        if l != str::parse::<i64>(&(part_b.clone()).unwrap()).unwrap() {
            panic!("day bounds failed: {}, {l} != {}", daynum, part_b.unwrap());
        }
    }
}
