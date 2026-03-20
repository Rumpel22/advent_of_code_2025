use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
enum Field {
    Start,
    Space,
    Splitter,
}
struct Manifold {
    fields: Vec<Field>,
    width: usize,
}

impl Manifold {
    fn row(&self, row: usize) -> Option<&[Field]> {
        let start = self.width * row;
        let end = self.width * (row + 1) - 1;
        if end < self.fields.len() {
            Some(&self.fields[start..=end])
        } else {
            None
        }
    }
    fn start(&self) -> usize {
        self.fields
            .iter()
            .position(|f| matches!(f, Field::Start))
            .unwrap()
    }
}

fn parse_input(input: &str) -> Manifold {
    let width = input.find('\n').unwrap();
    let fields = input
        .chars()
        .filter(char::is_ascii_graphic)
        .map(|c| match c {
            'S' => Field::Start,
            '^' => Field::Splitter,
            '.' => Field::Space,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    Manifold { fields, width }
}

fn calc_beam_splits(manifold: &Manifold) -> usize {
    let start = manifold.start();
    let mut beams = HashSet::from([start]);
    let mut splits = 0;
    let mut row = 1;
    while let Some(row_fields) = manifold.row(row + 1) {
        beams = beams
            .iter()
            .flat_map(|beam| {
                if row_fields[*beam] == Field::Splitter {
                    splits += 1;
                    [beam - 1, beam + 1]
                } else {
                    // Hack: return the same value twice, the HashSet will sort one of them out again
                    [*beam, *beam]
                }
            })
            .collect::<HashSet<_>>();
        row += 2;
    }
    splits
}

fn get_timelines(manifold: &Manifold) -> usize {
    let start = manifold.start();
    let mut timelines = HashMap::from([(start, 1)]);
    let mut row = 1;

    while let Some(row_fields) = manifold.row(row + 1) {
        let mut current = HashMap::new();
        for (index, count) in timelines {
            if row_fields[index] == Field::Splitter {
                *current.entry(index - 1).or_default() += count;
                *current.entry(index + 1).or_default() += count;
            } else {
                *current.entry(index).or_default() += count;
            }
        }
        timelines = current;
        row += 2;
    }

    timelines.values().sum()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let manifold = parse_input(input);
    let beam_splits = calc_beam_splits(&manifold);
    println!("Beam splits: {}", beam_splits);

    let timelines = get_timelines(&manifold);
    println!("Timelines: {}", timelines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let manifold = parse_input(input);
        let beam_splits = calc_beam_splits(&manifold);
        assert_eq!(21, beam_splits);
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo.txt");
        let manifold = parse_input(input);
        let timelines = get_timelines(&manifold);
        assert_eq!(40, timelines);
    }
}
