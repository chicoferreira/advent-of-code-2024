use aoc_runner_derive::aoc;
use fxhash::FxHashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let (mut a, mut b): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(str::split_whitespace)
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    a.sort_unstable();
    b.sort_unstable();

    a.iter().zip(&b).map(|(a, b)| a.abs_diff(*b)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let mut map = FxHashMap::default();
    let mut vec = Vec::new();

    for line in input.lines() {
        let mut line = line.split_whitespace();
        let a = line.next().unwrap().parse::<u64>().unwrap();
        let b = line.next().unwrap().parse::<u64>().unwrap();
        *map.entry(b).or_insert(0) += 1;
        vec.push(a);
    }

    vec.iter().map(|a| a * map.get(a).unwrap_or(&0)).sum()
}
