use aoc_runner_derive::aoc;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

type Pos = (i64, i64);

fn is_position_valid((x, y): Pos, (size_x, size_y): Pos) -> bool {
    x >= 0 && x <= size_x && y >= 0 && y <= size_y
}

fn parse_antennas(input: &str) -> (Pos, Vec<Vec<Pos>>) {
    let mut size_x = 0;
    let mut size_y = 0;

    let mut antennas = FxHashMap::default();
    for (y, line) in input.lines().enumerate() {
        size_y = y as i64;
        for (x, c) in line.chars().enumerate() {
            size_x = x as i64;
            if c == '.' {
                continue;
            }
            antennas
                .entry(c)
                .or_insert_with(Vec::new)
                .push((x as i64, y as i64));
        }
    }

    ((size_x, size_y), antennas.into_values().collect_vec())
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (bounds, antennas) = parse_antennas(input);
    let mut visited = FxHashSet::default();
    for positions in antennas {
        for (&pos1, &pos2) in positions.iter().tuple_combinations::<(_, _)>() {
            let (x1, y1) = pos1;
            let (x2, y2) = pos2;
            let (dx, dy) = (x2 - x1, y2 - y1);

            let upper = (x1 - dx, y1 - dy);
            let lower = (x2 + dx, y2 + dy);

            if is_position_valid(upper, bounds) {
                visited.insert(upper);
            }
            if is_position_valid(lower, bounds) {
                visited.insert(lower);
            }
        }
    }
    visited.len()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let (bounds, antennas) = parse_antennas(input);
    let mut visited = FxHashSet::default();
    for positions in antennas {
        for (&pos1, &pos2) in positions.iter().tuple_combinations::<(_, _)>() {
            let (mut x1, mut y1) = pos1;
            let (mut x2, mut y2) = pos2;
            let (dx, dy) = (x2 - x1, y2 - y1);

            while is_position_valid((x1, y1), bounds) {
                visited.insert((x1, y1));
                x1 -= dx;
                y1 -= dy;
            }

            while is_position_valid((x2, y2), bounds) {
                visited.insert((x2, y2));
                x2 += dx;
                y2 += dy;
            }
        }
    }
    visited.len()
}
