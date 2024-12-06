use std::{cmp::Ordering, str::Lines};

use aoc_runner_derive::aoc;
use fxhash::FxHashSet;

fn parse_number_line(line: &str) -> Vec<u32> {
    line.split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
}

fn parse_rules(iter: &mut Lines<'_>) -> FxHashSet<(u32, u32)> {
    let mut rules = FxHashSet::default();
    for line in iter {
        if line.is_empty() {
            break;
        }
        let left = line[0..2].parse().unwrap();
        let right = line[3..5].parse().unwrap();
        rules.insert((left, right));
    }
    rules
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let mut iter = input.lines();
    let rules = parse_rules(&mut iter);

    iter.map(parse_number_line)
        .filter(|numbers| numbers.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
        .map(|numbers| numbers[numbers.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    let mut iter = input.lines();
    let rules = parse_rules(&mut iter);

    iter.map(parse_number_line)
        .filter(|numbers| !numbers.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
        .map(|mut numbers| {
            numbers.sort_unstable_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            numbers[numbers.len() / 2]
        })
        .sum()
}
