use aoc_runner_derive::aoc;

fn rest_match(chars: &mut std::str::Chars, rest: &str) -> bool {
    rest.chars().all(|c| chars.next() == Some(c))
}

fn run(input: &str, can_turn_off: bool) -> u32 {
    enum ParseState {
        Mult,
        FirstNumber,
        SecondNumber,
        Off,
    }

    let mut state = ParseState::Mult;
    let mut chars = input.chars();

    let mut first_number = 0;
    let mut second_number = 0;

    let mut result = 0;

    while let Some(c) = chars.next() {
        match state {
            ParseState::Mult => {
                if c == 'm' {
                    if rest_match(&mut chars, "ul(") {
                        state = ParseState::FirstNumber;
                        first_number = 0;
                        second_number = 0;
                    }
                } else if can_turn_off && c == 'd' && rest_match(&mut chars, "on't()") {
                    state = ParseState::Off;
                }
            }
            ParseState::FirstNumber => {
                if let Some(n) = c.to_digit(10) {
                    first_number *= 10;
                    first_number += n;
                } else if c == ',' {
                    state = ParseState::SecondNumber;
                } else {
                    state = ParseState::Mult;
                }
            }
            ParseState::SecondNumber => {
                if let Some(n) = c.to_digit(10) {
                    second_number *= 10;
                    second_number += n;
                } else if c == ')' {
                    result += first_number * second_number;
                    state = ParseState::Mult;
                } else {
                    state = ParseState::Mult;
                }
            }
            ParseState::Off => {
                if c == 'd' && rest_match(&mut chars, "o()") {
                    state = ParseState::Mult;
                }
            }
        }
    }
    result
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    run(input, false)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    run(input, true)
}
