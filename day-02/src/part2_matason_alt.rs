const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> u64 {
    input.trim().split(',').fold(0, |mut acc, range| {
        let endpoints = range.split_once('-').unwrap();
        let start = endpoints.0.parse::<u64>().unwrap();
        let end = endpoints.1.parse::<u64>().unwrap();

        'outer: for id in start..=end {
            let number = id.to_string();
            for i in 1..number.len() {
                if number.len().is_multiple_of(i) {
                    let prefix = &number[0..i];
                    let mut suffix = &number[i..];
                    while let Some(tail) = suffix.strip_prefix(prefix) {
                        if tail.is_empty() {
                            acc += id;
                            continue 'outer;
                        }
                        suffix = tail;
                    }
                }
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {

    use super::solve;
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
