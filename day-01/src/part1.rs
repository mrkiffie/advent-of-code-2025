const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

const MAX: i32 = 100;

fn solve(input: &str) -> i32 {
    let mut point = 50;
    let mut zeroes = 0;
    for line in input.lines() {
        let (dir, count) = line.split_at(1);
        let count = count.parse::<i32>().unwrap() % MAX;
        match dir {
            "R" => point += count,
            "L" => point -= count,
            _ => unreachable!(),
        }
        if point >= MAX {
            point %= MAX;
        } else if point < 0 {
            point += MAX;
        }
        if point == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
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
