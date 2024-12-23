use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Clone, Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

fn parse_equation(input: &str) -> Option<Equation> {
    let (result, numbers) = input.split_once(":")?;
    let result = result.parse().ok()?;
    let numbers = numbers.split(" ").filter_map(|n| n.parse().ok()).collect();

    Some(Equation { result, numbers })
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Equation> {
    input.lines().filter_map(parse_equation).collect()
}

fn is_valid(equation: &Equation, concat: bool) -> bool {
    let mut stack: VecDeque<(u64, &[u64])> = VecDeque::new();
    stack.push_back((equation.result, equation.numbers.as_ref()));

    while let Some((result, numbers)) = stack.pop_front() {
        let Some((n, numbers)) = numbers.split_last() else {
            if result == numbers.iter().sum() {
                return true;
            } else {
                continue;
            }
        };

        match result.cmp(n) {
            std::cmp::Ordering::Less => {} // can't be valid
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                stack.push_back((result - n, numbers));

                if result % n == 0 {
                    stack.push_back((result / n, numbers));
                }

                if concat {
                    if let Some(result) = strip_matching_suffix(result, *n) {
                        stack.push_back((result, numbers));
                    }
                }
            }
        }
    }

    false
}

fn strip_matching_suffix(a: u64, b: u64) -> Option<u64> {
    if b == 0 {
        return if a % 10 == 0 { Some(a / 10) } else { None };
    }

    let mut temp_b = b;
    let mut divisor = 1;
    while temp_b >= 10 {
        temp_b /= 10;
        divisor *= 10;
    }
    divisor *= 10;

    let a_suffix = a % divisor;

    if a_suffix == b {
        Some(a / divisor)
    } else {
        None
    }
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|e| is_valid(e, false))
        .map(|e| e.result)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|e| is_valid(e, true))
        .map(|e| e.result)
        .sum()
}
