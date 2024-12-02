use std::collections::BinaryHeap;
use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (left, right): (BinaryHeap<_>, BinaryHeap<_>) = input
        .lines()
        .map(|s| s.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .unzip();
    left.into_sorted_vec()
        .iter()
        .zip(right.into_sorted_vec())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut right_counts = HashMap::new();
    input
        .lines()
        .flat_map(|s| s.split_once("   "))
        .map(|(l, r)| {
            (
                l.parse::<u32>().expect("Should parse"),
                r.parse::<u32>().expect("Should parse"),
            )
        })
        .map(|(l, r)| {
            right_counts
                .entry(r)
                .and_modify(|e| *e += 1)
                .or_insert(1u32);
            l
        })
        .collect::<Vec<u32>>()
        .iter()
        .map(|l| l * right_counts.get(l).unwrap_or(&0u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    #[test]
    fn sample1() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(test_input), 11);
        assert_eq!(part2(test_input), 31);
    }
}
