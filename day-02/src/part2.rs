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
    let s = id.to_string();
    // println!();
    // invalid if any of the substrings repeat at least twice
    // exhaustively check different substring lengths for repeats
    // chunk sizes only need to be up to half the length
    (1..=s.len() / 2)
        .rev()
        .map(|chunk_size| repeats(&s, chunk_size))
        .all(|b| !b)
}

/// Returns true is slice repeats at the `chunk_size`
fn repeats(s: &str, chunk_size: usize) -> bool {
    // println!("checking {s} for {chunk_size}");
    if s.len().is_multiple_of(chunk_size) {
        let mut i = chunk_size;
        while i < s.len() {
            if s[0..chunk_size] != s[i..i + chunk_size] {
                // println!("{s} stops repeating on iteration {i} for size {chunk_size}");
                return false;
            }
            i += chunk_size;
        }
        // println!("{s} valid chunk size {chunk_size}");
        return true;
    }
    // println!("{s} invalid chunk size {chunk_size}");
    false
}

#[cfg(test)]
mod tests {

    use super::{is_product_id_valid, repeats, solve};

    #[test]
    fn test_repeats() {
        assert!(repeats("123123", 3));
        assert!(!repeats("123123", 1));
        assert!(!repeats("123123", 2));
        assert!(repeats("1010", 2));
    }

    #[test]
    fn test_is_product_id_valid() {
        assert!(!is_product_id_valid(11));
        assert!(!is_product_id_valid(22));
        assert!(!is_product_id_valid(99));
        assert!(!is_product_id_valid(111));
        assert!(!is_product_id_valid(999));
        assert!(!is_product_id_valid(1010));
        assert!(!is_product_id_valid(1_188_511_885));
        assert!(!is_product_id_valid(222_222));
        assert!(!is_product_id_valid(446_446));
        assert!(!is_product_id_valid(38_593_859));
        assert!(!is_product_id_valid(565_656));
        assert!(!is_product_id_valid(824_824_824));
        assert!(!is_product_id_valid(2_121_212_121));

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
        assert_eq!(result, 4_174_379_265);
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

