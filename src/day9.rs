use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut disk: Vec<_> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 2 == 0 {
                vec![Some(i / 2); c as usize]
            } else {
                vec![None; c as usize]
            }
        })
        .collect();

    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        if disk[left].is_some() {
            left += 1;
        } else if disk[right].is_none() {
            right -= 1;
        } else {
            disk.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, &n)| n.map(|n| i * n as usize))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut disk: Vec<(usize, Option<usize>)> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, c)| (c as usize, if i % 2 == 0 { Some(i / 2) } else { None }))
        .collect();

    let mut right = disk.len() - 1;
    while right > 0 {
        let (amount, content_to_move) = disk[right];
        if content_to_move.is_none() {
            right -= 1;
        } else {
            for i in 0..right {
                let (free_size, content) = disk[i];
                if content.is_none() {
                    if free_size > amount {
                        disk[right] = (amount, None);
                        disk[i] = (amount, content_to_move);
                        disk.insert(i + 1, (free_size - amount, None));
                        break;
                    }

                    if free_size == amount {
                        disk[right] = (amount, None);
                        disk[i] = (amount, content_to_move);
                        break;
                    }
                }
            }
            right -= 1;
        }
    }

    let mut sum = 0;
    let mut current_index = 0;
    for (amount, content) in disk {
        match content {
            Some(content) => {
                for _ in 0..amount {
                    sum += content * current_index;
                    current_index += 1;
                }
            }
            None => current_index += amount,
        }
    }
    sum
}
