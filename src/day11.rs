use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;

fn split_number(number: usize) -> Option<(usize, usize)> {
    let digits = number.ilog10() + 1;
    if digits % 2 != 0 {
        return None;
    }

    let divisor = 10_usize.pow(digits / 2);
    Some((number / divisor, number % divisor))
}

fn blink_number(number: usize, old_count: usize, numbers: &mut HashMap<usize, usize>) {
    if number == 0 {
        *numbers.entry(1).or_default() += old_count;
    } else if let Some((left, right)) = split_number(number) {
        *numbers.entry(left).or_default() += old_count;
        *numbers.entry(right).or_default() += old_count;
    } else {
        *numbers.entry(number * 2024).or_default() += old_count;
    }
}

fn blink(numbers: &mut HashMap<usize, usize>) {
    let mut new = HashMap::new();
    for (number, count) in numbers.drain() {
        blink_number(number, count, &mut new);
    }
    std::mem::swap(&mut new, numbers);
}

fn parse(input: &str) -> HashMap<usize, usize> {
    input
        .split_whitespace()
        .filter_map(|line| line.parse().ok())
        .counts()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let mut numbers = parse(input);
    for _ in 0..25 {
        blink(&mut numbers);
    }
    numbers.values().sum()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let mut numbers = parse(input);
    for _ in 0..75 {
        blink(&mut numbers);
    }
    numbers.values().sum()
}
