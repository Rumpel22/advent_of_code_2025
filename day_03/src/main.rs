fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn max_digit(bank: &[u8]) -> (usize, &u8) {
    bank.iter()
        .enumerate()
        .max_by(|x, y| match x.1.cmp(y.1) {
            std::cmp::Ordering::Equal => y.0.cmp(&x.0),
            r => r,
        })
        .unwrap()
}

fn max_joltage(bank: &[u8], digits: usize) -> u64 {
    let range = &bank[..bank.len() - digits + 1];
    let (index, digit) = max_digit(range);
    let remaining_range = &bank[(index + 1)..];

    let first = (*digit as u64) * 10u64.pow((digits - 1) as u32);
    if digits == 1 {
        first
    } else {
        first + max_joltage(remaining_range, digits - 1)
    }
}

fn sum_joltages(banks: &[Vec<u8>], digits: usize) -> u64 {
    banks.iter().map(|bank| max_joltage(bank, digits)).sum()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let banks = parse_input(input);

    let solution = sum_joltages(&banks, 2);
    println!("{}", solution);
    let solution = sum_joltages(&banks, 12);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let banks = parse_input(input);

        assert_eq!(sum_joltages(&banks, 2), 357);
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo.txt");
        let banks = parse_input(input);

        assert_eq!(sum_joltages(&banks, 12), 3121910778619);
    }
}
