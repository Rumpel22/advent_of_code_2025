use std::{iter::from_fn, ops::RangeInclusive};

type Range = RangeInclusive<u64>;

fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|line| {
            let mut iter = line.split('-');
            let start: u64 = iter.next().unwrap().parse().unwrap();
            let end: u64 = iter.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect::<Vec<_>>()
}

fn nb_digits(n: &u64) -> u32 {
    if n == &0 { 0 } else { 1 + nb_digits(&(n / 10)) }
}

fn get_digits(n: &u64, digits: u32) -> u64 {
    let nb_digits = nb_digits(n);
    n / 10_u64.pow(nb_digits - digits)
}

fn repeat_digits(digits: &u64, r: u32) -> u64 {
    let nb_digits = nb_digits(digits);
    let factor = 10_u64.pow(nb_digits);
    (1..r).fold(*digits, |number, _| number * factor + digits)
}

fn invalid_ids(range: &Range) -> impl Iterator<Item = u64> {
    let digits = nb_digits(range.start());
    let power = digits / 2;

    let mut start = if digits % 2 == 0 {
        *range.start() / 10_u64.pow(power)
    } else {
        10_u64.pow(power)
    };
    if start * 10_u64.pow(nb_digits(&start)) + start < *range.start() {
        start += 1;
    }

    let mut n = start;
    from_fn(move || {
        let x = n * 10_u64.pow(nb_digits(&n)) + n;
        n += 1;

        if range.contains(&x) { Some(x) } else { None }
    })
}

fn sum_invalid_ids(ranges: &[Range]) -> u64 {
    ranges.iter().flat_map(|r| invalid_ids(r)).sum()
}

fn is_invalid_id(n: &u64) -> bool {
    let digits = nb_digits(n);
    (1..=(digits / 2)).filter(|i| digits % i == 0).any(|i| {
        let first_n = get_digits(n, i);
        let repetitions = digits / i;
        let number = repeat_digits(&first_n, repetitions);
        n == &number
    })
}

fn invalid_ids_2(range: &Range) -> impl Iterator<Item = u64> {
    range
        .clone()
        .filter_map(|x| if is_invalid_id(&x) { Some(x) } else { None })
}

fn sum_invalid_ids_2(ranges: &[Range]) -> u64 {
    ranges
        .iter()
        .flat_map(|r| invalid_ids_2(r))
        // .filter_map(|x| x)
        .sum()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let ranges = parse_input(input);

    let solution = sum_invalid_ids(&ranges);
    println!("{}", solution);

    let solution_2 = sum_invalid_ids_2(&ranges);
    println!("{}", solution_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1-3,5-7,10-15";
        let expected = vec![1..=3, 5..=7, 10..=15];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_sum_invalid_ids() {
        let input = include_str!("../input/demo.txt");
        let ranges = parse_input(input);

        assert_eq!(sum_invalid_ids(&ranges), 1227775554);
    }

    #[test]
    fn test_single_invalid_ids() {
        // According to problem desciption, the following IDs are invalid:
        let invalid_ids = vec![11, 22, 1010, 1188511885, 222222, 446446, 38593859];

        for &id in &invalid_ids {
            let ranges = [id..=id];
            assert_eq!(sum_invalid_ids(&ranges), id);
        }
    }

    #[test]
    fn test_nb_digits() {
        assert_eq!(nb_digits(&1), 1);
        assert_eq!(nb_digits(&9), 1);
        assert_eq!(nb_digits(&10), 2);
        assert_eq!(nb_digits(&99), 2);
        assert_eq!(nb_digits(&100), 3);
    }

    #[test]
    fn test_not_working_why() {
        let range = 1770..=2452;
        assert_eq!(sum_invalid_ids(&[range]), 14847);
    }

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(&1, 1), 1);
        assert_eq!(get_digits(&9, 1), 9);
        assert_eq!(get_digits(&43, 1), 4);
        assert_eq!(get_digits(&43, 2), 43);
    }

    #[test]
    fn test_sum_invalid_ids_2() {
        let input = include_str!("../input/demo.txt");
        let ranges = parse_input(input);

        assert_eq!(sum_invalid_ids_2(&ranges), 4174379265);
    }

    #[test]
    fn test_repeat_digits() {
        assert_eq!(repeat_digits(&123, 1), 123);
        assert_eq!(repeat_digits(&123, 2), 123123);
        assert_eq!(repeat_digits(&123, 3), 123123123);
    }

    #[test]
    fn test_single_invalid_ids_2() {
        // According to problem desciption, the following IDs are invalid:
        let invalid_ids = vec![
            11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
            2121212121,
        ];

        for &id in &invalid_ids {
            let ranges = [id..=id];
            assert_eq!(sum_invalid_ids_2(&ranges), id);
        }
    }
}
