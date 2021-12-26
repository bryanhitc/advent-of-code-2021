use std::str::Lines;

type Answer = u64;

pub fn part_one(lines: &Lines) -> Answer {
    let mut stack = Vec::new();
    let mut mismatches = Vec::new();
    let mut score = 0;

    for line in lines.clone() {
        stack.clear();
        mismatches.clear();

        for c in line.chars() {
            if mismatches.len() > 1 {
                break;
            }

            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                ')' => {
                    let char = stack.pop().unwrap();
                    if char != '(' {
                        mismatches.push(c);
                    }
                }
                ']' => {
                    let char = stack.pop().unwrap();
                    if char != '[' {
                        mismatches.push(c);
                    }
                }
                '}' => {
                    let char = stack.pop().unwrap();
                    if char != '{' {
                        mismatches.push(c);
                    }
                }
                '>' => {
                    let char = stack.pop().unwrap();
                    if char != '<' {
                        mismatches.push(c);
                    }
                }
                _ => {}
            }
        }

        // is_corrupt
        if mismatches.len() == 1 {
            score += match mismatches[0] {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("unknown mismatch symbol"),
            };
        }
    }

    score
}

pub fn part_two(lines: &Lines) -> Answer {
    let mut stack = Vec::new();
    let mut scores = Vec::new();

    for line in lines.clone() {
        let mut is_incomplete = true;
        stack.clear();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                ')' => {
                    let char = stack.pop().unwrap();
                    if char != '(' {
                        is_incomplete = false;
                        break;
                    }
                }
                ']' => {
                    let char = stack.pop().unwrap();
                    if char != '[' {
                        is_incomplete = false;
                        break;
                    }
                }
                '}' => {
                    let char = stack.pop().unwrap();
                    if char != '{' {
                        is_incomplete = false;
                        break;
                    }
                }
                '>' => {
                    let char = stack.pop().unwrap();
                    if char != '<' {
                        is_incomplete = false;
                        break;
                    }
                }
                _ => {}
            }
        }

        // is_corrupt
        if !is_incomplete {
            continue;
        }

        scores.push(stack.iter().rev().fold(0, |total, &char| {
            (total * 5)
                + match char {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("unknown mismatch symbol"),
                }
        }));
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

pub fn parse_input(input: &str) -> Lines {
    input.lines()
}

part_impl! {
    part_one_test01: 26397,
    part_one_problem: 389589,
    part_two_test01: 288957,
    part_two_problem: 1190420163,
}
