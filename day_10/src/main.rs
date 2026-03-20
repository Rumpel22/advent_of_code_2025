mod parse;

use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;
use std::{iter, str::FromStr};

#[derive(PartialEq, Debug, Clone)]
enum Led {
    On,
    Off,
}

type Buttons = DMatrix<u32>;
type Presses = DVector<u32>;
type Joltages = DVector<u32>;

#[derive(PartialEq, Debug)]
struct Machine {
    leds: Vec<Led>,
    buttons: Buttons,
    joltages: Joltages,
}

fn get_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| Machine::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

fn possibilities(nb_buttons: usize) -> impl ParallelIterator<Item = DVector<u32>> {
    (0..(2_u32.pow(nb_buttons as u32)))
        .into_par_iter()
        .map(move |i| {
            DVector::from_iterator(
                nb_buttons,
                iter::successors(Some(i), |i| Some(i / 2))
                    .map(|i| i % 2)
                    .take(nb_buttons),
            )
        })
}

fn is_solution(v: &DVector<u32>, leds: &[Led]) -> bool {
    v.iter()
        .zip(leds.iter())
        .all(|(x, led)| (x == &1 && led == &Led::On) || (x == &0 && led == &Led::Off))
}

fn get_new_state(buttons: &Buttons, presses: &DVector<u32>) -> DVector<u32> {
    buttons * presses
}

fn find_led_solutions(buttons: &Buttons, leds: &[Led]) -> impl ParallelIterator<Item = Presses> {
    possibilities(buttons.ncols()).filter(move |presses| {
        let solution = get_new_state(&buttons, &presses).apply_into(|x| *x = *x % 2);
        is_solution(&solution, leds)
    })
}

fn find_led_solution(machine: &Machine) -> u32 {
    find_led_solutions(&machine.buttons, &machine.leds)
        .map(|solution| solution.sum())
        .min()
        .unwrap()
}

fn find_joltages_solution(buttons: &Buttons, joltages: &Joltages) -> Option<u32> {
    // Solution inspired by https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    let leds = joltages
        .iter()
        .map(|x| if x % 2 == 0 { Led::Off } else { Led::On })
        .collect::<Vec<_>>();

    find_led_solutions(buttons, &leds)
        .filter_map(|solution| {
            let solution_length = solution.sum();
            let subtraction = get_new_state(buttons, &solution);
            if joltages >= &subtraction {
                let joltages = joltages - subtraction;
                if joltages.iter().all(|x| *x == 0) {
                    return Some(solution_length);
                }

                let joltages =
                    DVector::from_iterator(joltages.nrows(), joltages.iter().map(|x| x / 2));
                return find_joltages_solution(buttons, &joltages)
                    .map(|part_result| 2 * part_result + solution_length);
            }
            None
        })
        .min()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let machines = get_machines(input);

    let button_presses = machines.iter().map(find_led_solution).sum::<u32>();
    println!(
        "You have to press {} buttons to set the LEDs.",
        button_presses
    );

    let button_presses = machines
        .iter()
        .filter_map(|machine| find_joltages_solution(&machine.buttons, &machine.joltages))
        .sum::<u32>();
    println!(
        "You have to press {} buttons to set the joltages.",
        button_presses
    );
}

#[cfg(test)]
mod test {
    use nalgebra::*;

    use super::*;

    const TEST_BUTTONS: SMatrix<u32, 4, 6> = nalgebra::matrix![
        0, 0, 0, 0, 1, 1;
        0, 1, 0, 0, 0, 1;
        0, 0, 1, 1, 1, 0;
        1, 1, 0, 1, 0, 0;
    ];

    #[test]
    fn parse_test() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let expected_machine = Machine {
            leds: vec![Led::Off, Led::On, Led::On, Led::Off],
            buttons: DMatrix::<u32>::from_column_slice(4, 6, TEST_BUTTONS.as_slice()),
            joltages: nalgebra::dvector![3, 5, 4, 7],
        };
        let machine = Machine::from_str(input);
        assert_eq!(machine, Ok(expected_machine));
    }

    #[test]
    fn demo_test_1() {
        let input = include_str!("../input/demo.txt");
        let machines = get_machines(input);
        let button_presses = machines.iter().map(find_led_solution).sum::<u32>();

        assert_eq!(button_presses, 7)
    }

    #[test]
    fn demo_test_2() {
        let input = include_str!("../input/demo.txt");
        let machines = get_machines(input);
        let button_presses = machines
            .iter()
            .filter_map(|machine| find_joltages_solution(&machine.buttons, &machine.joltages))
            .sum::<u32>();

        assert_eq!(button_presses, 33)
    }
}
