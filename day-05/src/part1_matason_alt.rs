use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            start..=end
        })
        .collect::<Vec<RangeInclusive<i64>>>();

    ingredients
        .lines()
        .filter_map(|ingredient| ingredient.parse::<i64>().ok())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32");
        assert_eq!(result, 3);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {}

    #[divan::bench()]
    fn bench_solve() {
        super::solve(INPUT);
    }
}
