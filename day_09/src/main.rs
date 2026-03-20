use std::{cmp::Reverse, ops::RangeInclusive};

#[derive(Debug)]
struct Coordinates {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Rectangle<'a> {
    c1: &'a Coordinates,
    c2: &'a Coordinates,
}

impl Rectangle<'_> {
    fn size(&self) -> u64 {
        (self.c1.x.abs_diff(self.c2.x) + 1) * (self.c1.y.abs_diff(self.c2.y) + 1)
    }
}

fn parse_input(input: &str) -> Vec<Coordinates> {
    input
        .lines()
        .map(|line| {
            if let Some((x, y)) = line.split_once(',') {
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                Coordinates { x, y }
            } else {
                unreachable!()
            }
        })
        .collect()
}

#[derive(Debug)]
enum Direction {
    Vertical(u64),
    Horizontal(u64),
}

#[derive(Debug)]
struct Line {
    range: RangeInclusive<u64>,
    direction: Direction,
}

fn to_lines(coordinates: &'_ [Coordinates]) -> Vec<Line> {
    let mut lines: Vec<_> = coordinates
        .windows(2)
        .map(|pair| make_line(&pair[0], &pair[1]))
        .collect();
    lines.push(make_line(&coordinates[0], &coordinates.last().unwrap()));
    lines
}

fn make_line(c1: &Coordinates, c2: &Coordinates) -> Line {
    if c1.x == c2.x {
        let start = c1.y.min(c2.y);
        let end = c1.y.max(c2.y);
        Line {
            direction: Direction::Vertical(c1.x),
            range: start..=end,
        }
    } else {
        let start = c1.x.min(c2.x);
        let end = c1.x.max(c2.x);
        Line {
            direction: Direction::Horizontal(c1.y),
            range: start..=end,
        }
    }
}

fn sort_rectangles(rectangles: &mut [Rectangle]) {
    rectangles.sort_by_key(|rectangle| Reverse(rectangle.size()));
}

fn rectangles(coordinates: &'_ [Coordinates]) -> Vec<Rectangle<'_>> {
    coordinates
        .iter()
        .enumerate()
        .flat_map(|(index, c1)| {
            coordinates[(index + 1)..]
                .iter()
                .map(|c2| Rectangle { c1, c2 })
        })
        .collect()
}

fn do_intersect(line: &Line, rectangle: &Rectangle) -> bool {
    let top = rectangle.c1.y.min(rectangle.c2.y);
    let bottom = rectangle.c1.y.max(rectangle.c2.y);
    let left = rectangle.c1.x.min(rectangle.c2.x);
    let right = rectangle.c1.x.max(rectangle.c2.x);

    match line.direction {
        Direction::Vertical(x) => {
            !(line.range.end() <= &top || line.range.start() >= &bottom)
                && ((left + 1)..right).contains(&x)
        }
        Direction::Horizontal(y) => {
            !(line.range.end() <= &left || line.range.start() >= &right)
                && ((top + 1)..bottom).contains(&y)
        }
    }
}

fn max_rectangle<'a>(rectangles: &'a [Rectangle], lines: &[Line]) -> &'a Rectangle<'a> {
    rectangles
        .iter()
        .filter(|rectangle| lines.iter().all(|line| !do_intersect(line, rectangle)))
        .next()
        .unwrap()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let coordinates = parse_input(input);
    let mut rectangles = rectangles(&coordinates);
    sort_rectangles(&mut rectangles);
    let max_rectangle_size = rectangles.first().unwrap().size();
    println!("Max. rectangle: {}", max_rectangle_size);

    let lines = to_lines(&coordinates);
    let max_rectangle_within = max_rectangle(&rectangles, &lines);
    println!(
        "Max. complete rectangle within: {}",
        max_rectangle_within.size()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let coordinates = parse_input(input);
        let mut rectangles = rectangles(&coordinates);
        sort_rectangles(&mut rectangles);
        let max_rectangle_size = rectangles.first().unwrap().size();
        assert_eq!(50, max_rectangle_size);
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo.txt");
        let coordinates = parse_input(input);
        let mut rectangles = rectangles(&coordinates);
        sort_rectangles(&mut rectangles);
        let lines = to_lines(&coordinates);
        // println!("{:?}", lines);
        let max_rectangle_within = max_rectangle(&rectangles, &lines);
        // println!("{:?}", max_rectangle_within);
        assert_eq!(24, max_rectangle_within.size());
    }
}
