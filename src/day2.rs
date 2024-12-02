use aoc_runner_derive::aoc;

fn is_valid(line: &str, mut tolerance: bool) -> bool {
    let numbers = line
        .split_ascii_whitespace()
        .map(|part| part.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    if numbers.len() < 2 {
        return true;
    }

    let ordering = numbers[0].cmp(&numbers[1]);
    let (mut pointer_a, mut pointer_b) = (0, 1);

    let check = |pointer_a, pointer_b| {
        let a: i64 = numbers[pointer_a];
        let b: i64 = numbers[pointer_b];
        a.cmp(&b) == ordering && (1..=3).contains(&a.abs_diff(b))
    };

    while pointer_b < numbers.len() {
        if check(pointer_a, pointer_b) {
            pointer_a = pointer_b;
            pointer_b += 1;
        } else if tolerance {
            pointer_b += 1;
            if pointer_b < numbers.len() && !check(pointer_a, pointer_b) {
                pointer_a += 1;
            }
            tolerance = false;
        } else {
            return false;
        }
    }

    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|line| is_valid(line, false)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|line| is_valid(line, true)).count()
}
