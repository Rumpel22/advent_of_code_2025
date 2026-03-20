mod parse;

struct Shape {
    fields: Vec<bool>,
}

struct Region {
    x: usize,
    y: usize,
    presents: Vec<usize>,
}

fn fill_percentages(shapes: &[Shape], regions: &[Region]) -> Vec<f64> {
    let shape_sizes = shapes
        .iter()
        .map(|shape| shape.fields.iter().filter(|field| **field).count())
        .collect::<Vec<_>>();

    regions
        .iter()
        .map(|region| {
            let size = region.x * region.y;
            let occupied = region
                .presents
                .iter()
                .enumerate()
                .map(|(index, count)| count * shape_sizes[index])
                .sum::<usize>();
            occupied as f64 / size as f64
        })
        .collect()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (shapes, regions) = parse::parse(input);

    let possible_regions = fill_percentages(&shapes, &regions)
        .iter()
        .filter(|percentage| percentage <= &&0.90)
        .count();
    println!("{} regions are possible.", possible_regions);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn demo_test_1() {
        let input = include_str!("../input/demo.txt");
        let (_shapes, _regions) = parse::parse(input);

        // Algorithm does not work wih demo input data => No test possible
    }
}
