use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day10)]
fn generator(input: &str) -> ((usize, usize), Vec<Vec<u32>>) {
    let grid_x = input.lines().count();
    let grid_y = input.lines().next().unwrap().chars().count();

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    ((grid_x, grid_y), grid)
}

fn check_trailhead(
    (x, y): (usize, usize),
    grid: &Vec<Vec<u32>>,
    (grid_x, grid_y): (usize, usize),
) -> usize {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut sum = 0;
    let mut visited = vec![(x, y)];

    let mut queue = VecDeque::new();
    queue.push_back((0, x, y));

    while let Some((height, x, y)) = queue.pop_front() {
        for (x_dir, y_dir) in DIRECTIONS {
            let (new_x, new_y) = (x as i32 + x_dir, y as i32 + y_dir);
            if new_x < 0 || new_x >= grid_x as i32 || new_y < 0 || new_y >= grid_y as i32 {
                continue;
            }

            let (new_x, new_y) = (new_x as usize, new_y as usize);
            if visited.contains(&(new_x, new_y)) {
                continue;
            }

            let new_height = grid[new_y][new_x];
            if new_height != height + 1 {
                continue;
            }

            visited.push((new_x, new_y));

            if new_height == 9 {
                sum += 1;
            } else {
                queue.push_back((new_height, new_x, new_y));
            }
        }
    }
    sum
}

#[aoc(day10, part1)]
fn part1(((grid_x, grid_y), grid): &((usize, usize), Vec<Vec<u32>>)) -> usize {
    (0..*grid_x)
        .map(|x| {
            (0..*grid_y)
                .filter(|&y| grid[y][x] == 0)
                .map(|y| check_trailhead((x, y), &grid, (*grid_x, *grid_y)))
                .sum::<usize>()
        })
        .sum()
}

fn check_unique_trailhead(
    (x, y): (usize, usize),
    grid: &Vec<Vec<u32>>,
    (grid_x, grid_y): (usize, usize),
) -> usize {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut sum = 0;

    let mut queue = VecDeque::new();
    queue.push_back((0, x, y, vec![(x, y)]));

    while let Some((height, x, y, visited)) = queue.pop_front() {
        for (x_dir, y_dir) in DIRECTIONS {
            let (new_x, new_y) = (x as i32 + x_dir, y as i32 + y_dir);
            if new_x < 0 || new_x >= grid_x as i32 || new_y < 0 || new_y >= grid_y as i32 {
                continue;
            }

            let (new_x, new_y) = (new_x as usize, new_y as usize);
            if visited.contains(&(new_x, new_y)) {
                continue;
            }

            let new_height = grid[new_y][new_x];
            if new_height != height + 1 {
                continue;
            }

            if new_height == 9 {
                sum += 1;
            } else {
                let mut visited = visited.clone();
                visited.push((new_x, new_y));
                queue.push_back((new_height, new_x, new_y, visited));
            }
        }
    }
    sum
}

#[aoc(day10, part2)]
fn part2(((grid_x, grid_y), grid): &((usize, usize), Vec<Vec<u32>>)) -> usize {
    (0..*grid_x)
        .map(|x| {
            (0..*grid_y)
                .filter(|&y| grid[y][x] == 0)
                .map(|y| check_unique_trailhead((x, y), &grid, (*grid_x, *grid_y)))
                .sum::<usize>()
        })
        .sum()
}
