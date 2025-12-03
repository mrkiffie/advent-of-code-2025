const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &[u8]) -> i32 {
    input.split(|c| *c == b'\n').map(find_joltage).sum()
}
fn find_joltage(input: &[u8]) -> i32 {
    let mut first = b'0';
    let mut second = b'0';
    for d in input {
        if second > first {
            first = second;
            second = b'0';
        }
        if *d > second {
            second = *d;
        }
    }
    i32::from(first - b'0') * 10 + i32::from(second - b'0')
}
#[cfg(test)]
mod tests {
    use super::{find_joltage, solve};

    #[test]
    fn test_find_joltage() {
        assert_eq!(find_joltage(b"987654321111111"), 98);
        assert_eq!(find_joltage(b"811111111111119"), 89);
        assert_eq!(find_joltage(b"234234234234278"), 78);
        assert_eq!(find_joltage(b"818181911112111"), 92);
    }

    #[test]
    fn test_solve() {
        let result = solve(b"987654321111111\n811111111111119\n234234234234278\n818181911112111");
        assert_eq!(result, 357);
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
