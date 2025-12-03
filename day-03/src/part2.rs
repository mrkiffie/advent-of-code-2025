const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &[u8]) -> u64 {
    input.split(|c| *c == b'\n').map(find_joltage).sum()
}
fn find_joltage(input: &[u8]) -> u64 {
    let mut digits = [b'0'; 12];
    for d in input {
        for i in 0..11 {
            if digits[i + 1] > digits[i] {
                digits[i] = digits[i + 1];
                digits[i + 1] = b'0';
            }
        }
        if *d > digits[11] {
            digits[11] = *d;
        }
    }
    digits
        .iter()
        .map(|d| u64::from(d - b'0'))
        .fold(0, |total, d| total * 10 + d)
}
#[cfg(test)]
mod tests {
    use super::{find_joltage, solve};

    #[test]
    fn test_find_joltage() {
        assert_eq!(find_joltage(b"987654321111111"), 987_654_321_111);
        assert_eq!(find_joltage(b"811111111111119"), 811_111_111_119);
        assert_eq!(find_joltage(b"234234234234278"), 434_234_234_278);
        assert_eq!(find_joltage(b"818181911112111"), 888_911_112_111);
    }

    #[test]
    fn test_solve() {
        let result = solve(b"987654321111111\n811111111111119\n234234234234278\n818181911112111");
        assert_eq!(result, 3_121_910_778_619);
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

