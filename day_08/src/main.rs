use std::{cmp::Reverse, mem::swap};

#[derive(Clone, PartialEq)]
struct Box {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Clone)]
struct Circuit<'a> {
    boxes: Vec<&'a Box>,
}

impl<'a> Circuit<'a> {
    fn contains(&self, b: &Box) -> bool {
        self.boxes.contains(&b)
    }
}

fn get_circuit<'a>(circuits: &'a Vec<Circuit<'a>>, b: &'a Box) -> Option<usize> {
    circuits.iter().position(|circuit| circuit.contains(b))
}

fn distance(b1: &Box, b2: &Box) -> u64 {
    b1.x.abs_diff(b2.x).pow(2) + b1.y.abs_diff(b2.y).pow(2) + b1.z.abs_diff(b2.z).pow(2)
}

fn parse_input(input: &str) -> Vec<Box> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split(',');
            let x = numbers.next().unwrap().parse::<_>().unwrap();
            let y = numbers.next().unwrap().parse::<_>().unwrap();
            let z = numbers.next().unwrap().parse::<_>().unwrap();
            Box { x, y, z }
        })
        .collect()
}

fn get_distances(boxes: &'_ [Box]) -> Vec<((&Box, &Box), u64)> {
    let mut distances = vec![];
    for (index, b1) in boxes.iter().enumerate() {
        for b2 in &boxes[index + 1..] {
            let dist = distance(b1, b2);
            distances.push(((b1, b2), dist));
        }
    }
    distances.sort_by_key(|(_, d)| *d);

    distances
}

fn connect_n_boxes<'a>(distances: &[((&'a Box, &'a Box), u64)], count: usize) -> Vec<Circuit<'a>> {
    let mut circuits = vec![];

    for ((b1, b2), _) in distances.iter().take(count) {
        update_circuits(&mut circuits, b1, b2);
    }

    circuits
}

fn connect_all_boxes<'a>(
    distances: &[((&'a Box, &'a Box), u64)],
    box_count: usize,
) -> (&'a Box, &'a Box) {
    let mut circuits = vec![];

    for ((b1, b2), _) in distances {
        update_circuits(&mut circuits, b1, b2);

        if circuits.len() == 1 && circuits[0].boxes.len() == box_count {
            return (b1, b2);
        }
    }
    unreachable!()
}

fn update_circuits<'a>(circuits: &mut Vec<Circuit<'a>>, b1: &'a Box, b2: &'a Box) {
    match (get_circuit(&*circuits, b1), get_circuit(&*circuits, b2)) {
        (None, None) => circuits.push(Circuit {
            boxes: vec![b1, b2],
        }),

        (None, Some(index)) => {
            let circuit = &mut circuits[index];
            circuit.boxes.push(b1);
        }
        (Some(index), None) => {
            let circuit = &mut circuits[index];
            circuit.boxes.push(b2);
        }

        (Some(c1_index), Some(c2_index)) if c1_index != c2_index => {
            let mut c2_boxes = vec![];
            swap(&mut c2_boxes, &mut circuits[c2_index].boxes);
            circuits[c1_index].boxes.extend_from_slice(&c2_boxes);

            // First update the combined circuit, then remove c2_index, or c1_index may become invalid
            circuits.remove(c2_index);
        }
        _ => {}
    }
}

fn get_max_distances(circuits: &[Circuit]) -> Vec<usize> {
    let mut circuits = circuits.to_vec();
    circuits.sort_by_key(|circuit| Reverse(circuit.boxes.len()));
    circuits.iter().map(|c| c.boxes.len()).collect()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let boxes = parse_input(input);
    let distances = get_distances(&boxes);
    let circuits: Vec<Circuit> = connect_n_boxes(&distances, 1000);
    let circuits = get_max_distances(&circuits);
    let result: usize = circuits.iter().take(3).product();

    println!("Result: {}", result);

    let last_two = connect_all_boxes(&distances, boxes.len());
    let result = last_two.0.x * last_two.1.x;
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let boxes = parse_input(input);
        let distances = get_distances(&boxes);
        let circuits = connect_n_boxes(&distances, 10);
        let circuits = get_max_distances(&circuits);
        let result: usize = circuits.iter().take(3).product();
        assert_eq!(40, result)
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo.txt");
        let boxes = parse_input(input);
        let distances = get_distances(&boxes);
        let last_two = connect_all_boxes(&distances, boxes.len());
        let result = last_two.0.x * last_two.1.x;
        assert_eq!(25272, result)
    }
}
