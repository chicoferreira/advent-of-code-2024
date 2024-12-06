use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &[u8]) -> usize {
    let len_x = input.iter().position(|&c| c == b'\n').unwrap() as i64;
    let len_y = input.len() as i64 / len_x;

    let get_char = |x, y| input.get((y * (len_x + 1) + x) as usize);

    let directions = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut count = 0;

    for y in 0..len_y {
        for x in 0..len_x {
            if get_char(x, y) == Some(&b'X') {
                for &direction in directions.iter() {
                    let c1 = get_char(x + direction.0, y + direction.1);
                    let c2 = get_char(x + direction.0 * 2, y + direction.1 * 2);
                    let c3 = get_char(x + direction.0 * 3, y + direction.1 * 3);

                    if c1 == Some(&b'M') && c2 == Some(&b'A') && c3 == Some(&b'S') {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(input: &[u8]) -> usize {
    let len_x = input.iter().position(|&c| c == b'\n').unwrap() as i64;
    let len_y = input.len() as i64 / len_x;

    let get_char = |x, y| input.get((y * (len_x + 1) + x) as usize);

    let mut count = 0;
    let is_ms_pair = |a, b| {
        matches!(
            (a, b),
            (Some(&b'M'), Some(&b'S')) | (Some(&b'S'), Some(&b'M'))
        )
    };

    for y in 0..len_y {
        for x in 0..len_x {
            if get_char(x, y) == Some(&b'A')
                && is_ms_pair(get_char(x - 1, y - 1), get_char(x + 1, y + 1))
                && is_ms_pair(get_char(x + 1, y - 1), get_char(x - 1, y + 1))
            {
                count += 1;
            }
        }
    }

    count
}
