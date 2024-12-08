use aoc_runner_derive::aoc;
use fxhash::FxHashSet;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn apply(&self, pos: Pos) -> Pos {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }

    fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn find_obstacle((x, y): Pos, obstacles: &[Pos], dir: Direction) -> Option<Pos> {
    let candidates = obstacles.iter().copied().filter(|&(o_x, o_y)| match dir {
        Direction::Up => x == o_x && y > o_y,
        Direction::Down => x == o_x && y < o_y,
        Direction::Left => y == o_y && x > o_x,
        Direction::Right => y == o_y && x < o_x,
    });

    match dir {
        Direction::Up => candidates.min_by_key(|(_, o_y)| y - o_y),
        Direction::Down => candidates.min_by_key(|(_, o_y)| o_y - y),
        Direction::Left => candidates.min_by_key(|(o_x, _)| x - o_x),
        Direction::Right => candidates.min_by_key(|(o_x, _)| o_x - x),
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> usize {
    let len_x = input.iter().position(|&c| c == b'\n').unwrap() as usize;
    let len_y = input.len() as usize / len_x;

    let mut obstacles: Vec<(usize, usize)> = vec![];
    let mut pos = (0, 0);
    let mut current_direction = Direction::Up;

    let mut visited = FxHashSet::default();

    for y in 0..len_y {
        for x in 0..len_x {
            match input[y * (len_x + 1) + x] {
                b'#' => obstacles.push((x, y)),
                b'^' => pos = (x, y),
                _ => {}
            }
        }
    }

    fn mark_path(from: Pos, to: Pos, visited: &mut FxHashSet<(usize, usize)>) {
        let (min_x, max_x) = (from.0.min(to.0), from.0.max(to.0));
        let (min_y, max_y) = (from.1.min(to.1), from.1.max(to.1));

        visited.extend((min_x..=max_x).flat_map(|x| (min_y..=max_y).map(move |y| (x, y))));
    }

    while let Some(obstacle) = find_obstacle(pos, &obstacles, current_direction) {
        let to = current_direction.invert().apply(obstacle);
        mark_path(to, pos, &mut visited);

        pos = to;
        current_direction = current_direction.rotate();
    }

    // add remaining steps to the border
    let last_pos = match current_direction {
        Direction::Up => (pos.0, 0),
        Direction::Down => (pos.0, len_y - 1),
        Direction::Left => (0, pos.1),
        Direction::Right => (len_x - 1, pos.1),
    };
    mark_path(pos, last_pos, &mut visited);

    visited.len()
}

fn check_obstacle_loop(mut pos: Pos, obstacles: &[Pos]) -> bool {
    let mut current_direction = Direction::Up;
    let mut visited = FxHashSet::default();
    while let Some(obstacle) = find_obstacle(pos, &obstacles, current_direction) {
        let to = current_direction.invert().apply(obstacle);
        let (min_x, max_x) = (pos.0.min(to.0), pos.0.max(to.0));
        let (min_y, max_y) = (pos.1.min(to.1), pos.1.max(to.1));

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !visited.insert((x, y, current_direction)) {
                    return true;
                }
            }
        }

        pos = to;
        current_direction = current_direction.rotate();
    }

    false
}

#[aoc(day6, part2)]
pub fn part2(input: &[u8]) -> usize {
    let len_x = input.iter().position(|&c| c == b'\n').unwrap() as usize;
    let len_y = input.len() as usize / len_x;

    let mut obstacles: Vec<(usize, usize)> = vec![];
    let mut pos = (0, 0);

    for y in 0..len_y {
        for x in 0..len_x {
            match input[y * (len_x + 1) + x] {
                b'#' => obstacles.push((x, y)),
                b'^' => pos = (x, y),
                _ => {}
            }
        }
    }

    let mut loops = 0;

    for y in 0..len_y {
        for x in 0..len_x {
            if !obstacles.contains(&(x, y)) {
                obstacles.push((x, y));
                if check_obstacle_loop(pos, &obstacles) {
                    loops += 1;
                }
                obstacles.pop();
            }
        }
    }

    loops
}
