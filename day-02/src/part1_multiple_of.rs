const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .filter_map(|range| range.split_once('-'))
        .map(|(lower, upper)| lower.parse::<u64>().unwrap()..=upper.parse::<u64>().unwrap())
        .flat_map(|range| range.filter(|id| !is_product_id_valid(*id)))
        .sum()
}

/// Returns true if product id is valid
fn is_product_id_valid(id: u64) -> bool {
    !match id.ilog10() {
        1 => id.is_multiple_of(11),
        3 => id.is_multiple_of(101),
        5 => id.is_multiple_of(1001),
        7 => id.is_multiple_of(10001),
        9 => id.is_multiple_of(100_001),
        _ => false,
    }
}

#[cfg(test)]
mod tests {

    use super::{is_product_id_valid, solve};

    #[test]
    fn test_is_product_id_valid() {
        // 2 digits
        assert!(!is_product_id_valid(11), "1 group of 2");
        // 4 digits
        assert!(!is_product_id_valid(1212), "2 groups of 2");
        // 6 digits
        assert!(!is_product_id_valid(123_123), "2 groups of 3");
        // 8 digits
        assert!(!is_product_id_valid(12_341_234), "2 groups of 4");
        // 10 digits
        assert!(!is_product_id_valid(1_234_512_345), "2 groups of 5");

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
