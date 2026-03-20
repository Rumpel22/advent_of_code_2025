fn main() {
    let input = include_str!("../input/input.txt");
    let turns = input
        .lines()
        .map(|s| {
            let (dir, dist) = s.split_at(1);
            let dist: i32 = dist.parse().unwrap();
            match dir {
                "R" => dist,
                "L" => -dist,
                _ => panic!("Invalid direction"),
            }
        })
        .collect::<Vec<_>>();

    let mut positions = turns
        .iter()
        .scan(50, |position, turn| {
            *position = (*position + turn).rem_euclid(100);
            Some(*position)
        })
        .collect::<Vec<_>>();

    let zeros = positions.iter().filter(|position| **position == 0).count();
    println!("The dial is at zero {} times.", zeros);

    positions.insert(0, 50);
    positions.pop();
    let passbys = positions
        .iter()
        .zip(turns.iter())
        .map(|(position, turn)| {
            // Turning right is easy. So if turning left is required, we mirror the position and turn right instead to get the same effect.
            let position = if turn >= &0 || position == &0 {
                *position
            } else {
                100 - *position
            };
            let turn = turn.abs();
            (position + turn) / 100
        })
        .sum::<i32>();
    println!("The dial passes by zero {} times.", passbys);
}
