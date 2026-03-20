struct Map {
    data: Vec<bool>,
    height: usize,
    width: usize,
}

impl Map {
    fn get(&self, x: &i32, y: &i32) -> Option<bool> {
        let x = *x as usize;
        let y = *y as usize;
        if (..self.width).contains(&x) && (..self.height).contains(&y) {
            let index = self.width * y + x;
            return Some(self.data[index]);
        }
        None
    }

    fn remove(&mut self, x: &i32, y: &i32) {
        let x = *x as usize;
        let y = *y as usize;
        let index = self.width * y + x;
        self.data[index] = false;
    }
}

fn parse_input(input: &str) -> Map {
    let width = input.find('\n').unwrap();
    let data = input
        .chars()
        .filter(|c| c.is_ascii_graphic())
        .map(|c| c == '@')
        .collect::<Vec<_>>();
    let height = data.len() / width;
    Map {
        data,
        height,
        width,
    }
}

fn find_accessible_rolls(map: &Map) -> Vec<(i32, i32)> {
    const ADJACENTS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    map.data
        .iter()
        .enumerate()
        .filter_map(|(index, paper)| {
            if *paper {
                let x = (index % map.width) as i32;
                let y = (index / map.width) as i32;
                Some((x, y))
            } else {
                None
            }
        })
        .filter_map(|(x, y)| {
            if ADJACENTS
                .iter()
                .filter(|(dx, dy)| {
                    let x = x + dx;
                    let y = y + dy;
                    map.get(&x, &y).unwrap_or(false)
                })
                .count()
                < 4
            {
                Some((x, y))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn remove_accessible_rolls(map: &mut Map) -> usize {
    let mut counter = 0;
    loop {
        let to_remove = find_accessible_rolls(map);
        counter += to_remove.len();
        if to_remove.is_empty() {
            break;
        }
        to_remove.iter().for_each(|(x, y)| map.remove(x, y));
    }
    counter
}

fn main() {
    let input = include_str!("../input/input.txt");
    let mut map = parse_input(input);
    let accessible_rolls = find_accessible_rolls(&map);
    println!("{}", accessible_rolls.len());

    let removed_rolls = remove_accessible_rolls(&mut map);
    println!("{}", removed_rolls);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let map = parse_input(input);
        let accessible_rolls = find_accessible_rolls(&map);
        assert_eq!(13, accessible_rolls.len());
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo.txt");
        let mut map = parse_input(input);
        let removed_rolls = remove_accessible_rolls(&mut map);
        assert_eq!(43, removed_rolls);
    }
}
