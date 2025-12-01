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
        let count = count.parse::<i32>().unwrap();
        let before = point;
        zeroes += count / MAX;
        let count = count % MAX;
        match dir {
            "R" => point += count,
            "L" => point -= count,
            _ => unreachable!(),
        }

        if point == 0 {
            zeroes += 1;
        } else if point >= MAX {
            zeroes += 1;
            point -= MAX;
        } else if point < 0 {
            if before != 0 {
                zeroes += 1;
            }
            point += MAX;
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
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let result = solve(
            "L47
L57
L46
L79
L21
L99
R88
R11",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let result = solve("L150");
        assert_eq!(result, 2);
        let result = solve("L1000");
        assert_eq!(result, 10);
        let result = solve("L1050");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_4() {
        let result = solve("R50");
        assert_eq!(result, 1);
        let result = solve("R1050");
        assert_eq!(result, 11);
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

