use std::collections::HashMap;

type Devices<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &'_ str) -> Devices<'_> {
    input
        .lines()
        .map(|line| {
            let input = &line[..3];
            let outputs = line[5..].split_ascii_whitespace().collect::<Vec<_>>();
            (input, outputs)
        })
        .collect()
}

fn sort_topological<'a>(devices: &'a Devices, start: &'a str) -> Vec<&'a str> {
    let mut order = vec![start];
    let mut index = 0;
    while index < order.len() {
        let current = order[index];
        if order[..index].contains(&current) {
            order.remove(index);
            continue;
        }

        for next in devices.get(current).unwrap_or(&vec![]) {
            match order.iter().take(index).position(|x| x == next) {
                Some(i) => {
                    order.remove(i);
                    index = i - 1;
                    break;
                }
                None => {
                    order.push(*next);
                }
            }
        }
        index += 1;
    }
    order
}

fn find_paths<'a>(range: &[&'a str], devices: &'a Devices) -> usize {
    // The devices form a directed acyclic graph (DAG) =>
    // - order the devices topologically
    // - just consider the relevant devices (the "range")

    let mut distances = HashMap::<&'a str, usize>::new();
    distances.insert(range[0], 1);

    for d in range {
        if !devices.contains_key(d) {
            continue;
        }
        for next in &devices[d] {
            *distances.entry(next).or_insert(0) += *distances.get(d).unwrap_or(&0);
        }
    }
    *distances.get(range.last().unwrap()).unwrap_or(&0)
}

fn find_you_path_count(devices: &Devices) -> usize {
    let order = sort_topological(devices, "you");

    let start = order.iter().position(|x| x == &"you").unwrap();
    let end = order.iter().position(|x| x == &"out").unwrap();
    find_paths(&order[start..=end], devices)
}

fn find_svr_path_count(devices: &Devices) -> usize {
    let order = sort_topological(devices, "svr");

    let svr_index = order.iter().position(|x| x == &"svr").unwrap();
    let dac_index = order.iter().position(|x| x == &"dac").unwrap();
    let fft_index = order.iter().position(|x| x == &"fft").unwrap();
    let out_index = order.iter().position(|x| x == &"out").unwrap();

    // either fft_dac or dac_fft must be empty (acyclic graph)

    let fft_dac = find_paths(&order[fft_index..=dac_index], devices);
    if fft_dac != 0 {
        let svr_fft = find_paths(&order[svr_index..=fft_index], devices);
        let dac_out = find_paths(&order[dac_index..=out_index], devices);
        return svr_fft * fft_dac * dac_out;
    }

    let svr_dac = find_paths(&order[svr_index..=dac_index], devices);
    let dac_fft = find_paths(&order[dac_index..=fft_index], devices);
    let fft_out = find_paths(&order[fft_index..=out_index], devices);

    svr_dac * dac_fft * fft_out
}

fn main() {
    let input = include_str!("../input/input.txt");
    let devices = parse_input(input);
    let path_count = find_you_path_count(&devices);
    println!("There are {} paths.", path_count);

    let path_count = find_svr_path_count(&devices);
    println!("There are {} paths.", path_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let devices = parse_input(input);
        let path_count = find_you_path_count(&devices);
        assert_eq!(5, path_count);
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo2.txt");
        let devices = parse_input(input);
        let path_count = find_svr_path_count(&devices);
        assert_eq!(2, path_count);
    }
}
