#[derive(Clone, Copy)]
enum Operation {
    Plus,
    Multiplication,
}
enum Item {
    Operation(Operation),
    Number(u64),
}

struct Problem {
    numbers: Vec<u64>,
    op: Operation,
}

fn parse_input(input: &str) -> Vec<Problem> {
    let (operations, numbers): (Vec<_>, Vec<_>) = input
        .split_ascii_whitespace()
        .map(|item| {
            if item == "*" {
                Item::Operation(Operation::Multiplication)
            } else if item == "+" {
                Item::Operation(Operation::Plus)
            } else {
                Item::Number(item.parse::<u64>().unwrap())
            }
        })
        .partition(|item| matches!(item, Item::Operation(_)));
    operations
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let numbers = numbers
                .iter()
                .skip(i)
                .step_by(operations.len())
                .map(|item| match item {
                    Item::Operation(_) => unreachable!(),
                    Item::Number(n) => *n,
                })
                .collect::<Vec<_>>();
            let op = match op {
                Item::Operation(operation) => *operation,
                Item::Number(_) => unreachable!(),
            };
            Problem { numbers, op }
        })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<Problem> {
    let line_length = input.find('\n').unwrap();
    let number_lines = input.len() / (line_length + 1);
    let number_input = &input[..(number_lines * (line_length + 1))];
    let op_input = &input[(number_lines * (line_length + 1))..];
    let mut problems = vec![];

    let mut numbers = vec![];
    let mut op = None;
    for index in 0..line_length {
        let number = number_input
            .chars()
            .skip(index)
            .step_by(line_length + 1)
            .map(|c| c.to_digit(10))
            .fold(0, |x, digit| match digit {
                Some(digit) => x * 10 + digit,
                None => x,
            });
        op = match op_input.chars().nth(index).unwrap() {
            '*' => Some(Operation::Multiplication),
            '+' => Some(Operation::Plus),
            _ => op,
        };
        if number > 0 {
            numbers.push(number as u64);
        } else {
            problems.push(Problem {
                numbers: numbers.clone(),
                op: op.unwrap(),
            });
            numbers.clear();
        }
    }
    problems.push(Problem {
        numbers: numbers,
        op: op.unwrap(),
    });

    problems
}

fn get_grand_total(problems: &[Problem]) -> u64 {
    problems
        .iter()
        .map(|problem| match problem.op {
            Operation::Plus => problem.numbers.iter().sum::<u64>(),
            Operation::Multiplication => problem.numbers.iter().product(),
        })
        .sum()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let problems = parse_input(input);
    let grand_total = get_grand_total(&problems);
    println!("Grand total: {}", grand_total);

    let problems = parse_input_2(input);
    let grand_total = get_grand_total(&problems);
    println!("Grand total 2: {}", grand_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let problems = parse_input(input);
        let grand_total = get_grand_total(&problems);
        assert_eq!(4277556, grand_total);
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo.txt");
        let problems = parse_input_2(input);
        let grand_total = get_grand_total(&problems);
        assert_eq!(3263827, grand_total);
    }
}
