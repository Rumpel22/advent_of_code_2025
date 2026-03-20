use std::ops::RangeInclusive;

struct MergedRanges {
    ranges: Vec<RangeInclusive<u64>>,
}

fn overlap_ranges(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

fn merge_adjacent_ranges(
    r1: &RangeInclusive<u64>,
    r2: &RangeInclusive<u64>,
) -> RangeInclusive<u64> {
    assert!(overlap_ranges(r1, r2));

    let start = r1.start().min(r2.start());
    let end = r1.end().max(r2.end());
    *start..=*end
}

impl MergedRanges {
    fn new() -> Self {
        MergedRanges { ranges: vec![] }
    }

    fn count(&self) -> u64 {
        self.ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
    }

    fn merge(&mut self, range: &RangeInclusive<u64>) {
        self.ranges.push(range.clone());
        self.ranges.sort_by(|a, b| a.start().cmp(b.start()));

        if let Some(overlap) = self
            .ranges
            .windows(2)
            .enumerate()
            .filter(|(_, pair)| {
                let l = &pair[0];
                let r = &pair[1];
                overlap_ranges(l, r)
            })
            .next()
        {
            // Remove both ranges and insert a new (combined) range
            let new_range = merge_adjacent_ranges(&overlap.1[0], &overlap.1[1]);
            let index = overlap.0;
            self.ranges.remove(index);
            self.ranges.remove(index);
            self.merge(&new_range);
        }
    }
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (range_input, id_input) = input.split_once("\n\n").unwrap();
    let ranges = range_input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();
    let ids = id_input
        .lines()
        .map(|id| id.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    (ranges, ids)
}

fn get_fresh_ids(ranges: &[RangeInclusive<u64>], ids: &[u64]) -> Vec<u64> {
    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .cloned()
        .collect::<Vec<_>>()
}

fn merge_ranges(ranges: &[RangeInclusive<u64>]) -> MergedRanges {
    let mut merged_ranges = MergedRanges::new();
    ranges.iter().for_each(|range| merged_ranges.merge(range));
    merged_ranges
}

fn total_fresh_ids(ranges: &[RangeInclusive<u64>]) -> u64 {
    let merged_ranges = merge_ranges(ranges);
    merged_ranges.count()
}

fn main() {
    let input = include_str!("../input/input.txt");
    let (ranges, ids) = parse_input(input);
    let fresh_ids = get_fresh_ids(&ranges, &ids);
    println!("Fresh IDs: {}", fresh_ids.len());

    let fresh_id_count = total_fresh_ids(&ranges);
    println!("Total fresh IDs: {}", fresh_id_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
        let input = include_str!("../input/demo.txt");
        let (ranges, ids) = parse_input(input);
        let fresh_ids = get_fresh_ids(&ranges, &ids);
        assert_eq!(3, fresh_ids.len());
    }

    #[test]
    fn test_demo_2() {
        let input = include_str!("../input/demo.txt");
        let (ranges, _) = parse_input(input);
        let fresh_id_count = total_fresh_ids(&ranges);
        assert_eq!(14, fresh_id_count);
    }

    #[test]
    fn test_count() {
        let mut ranges = MergedRanges::new();
        assert_eq!(0, ranges.count());
        ranges.merge(&(1..=1));
        assert_eq!(1, ranges.count());
        ranges.merge(&(3..=5));
        assert_eq!(4, ranges.count());
    }
}
