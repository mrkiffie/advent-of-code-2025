use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> u64 {
    let ranges = parse_input(input);
    ranges.into_iter().map(count_invalid_ids).sum()
}
fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    let mut ranges = vec![];
    for range in input.trim().split(',') {
        if let Some((lower, upper)) = range.split_once('-')
            && let (Ok(lower), Ok(upper)) = (lower.parse(), upper.parse())
        {
            ranges.push(lower..=upper);
        }
    }
    ranges
}

fn count_invalid_ids(range: RangeInclusive<u64>) -> u64 {
    range.filter(|id| !is_product_id_valid(*id)).sum()
}

/// Returns true if product id is valid
fn is_product_id_valid(id: u64) -> bool {
    debug_assert_ne!(id, 0);
    let digits = id.ilog10() + 1;

    // if odd
    if digits & 1 == 1 {
        true
    } else {
        let divisor = 10_u64.pow(digits / 2);
        id / divisor != id % divisor
    }
}

#[cfg(test)]
mod tests {

    use super::{is_product_id_valid, solve};

    #[test]
    fn test_is_product_id_valid() {
        assert!(!is_product_id_valid(11));
        assert!(!is_product_id_valid(22));
        assert!(!is_product_id_valid(99));
        assert!(!is_product_id_valid(1010));
        assert!(!is_product_id_valid(1_188_511_885));
        assert!(!is_product_id_valid(222_222));
        assert!(!is_product_id_valid(446_446));
        assert!(!is_product_id_valid(38_593_859));

        assert!(is_product_id_valid(12));
        assert!(is_product_id_valid(23));
        assert!(is_product_id_valid(100));
        assert!(is_product_id_valid(1011));
        assert!(is_product_id_valid(1_188_511_886));
        assert!(is_product_id_valid(222_223));
        assert!(is_product_id_valid(446_447));
        assert!(is_product_id_valid(38_593_860));
    }

    #[test]
    fn test_solve() {
        let result = solve(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(result, 1_227_775_554);
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
