use glam::IVec2;
use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> i32 {
    let lookup = parse_input(input);
    let mut total = 0;
    for roll in &lookup {
        let count = [
            IVec2::new(-1, -1),
            IVec2::new(0, -1),
            IVec2::new(1, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(-1, 1),
            IVec2::new(0, 1),
            IVec2::new(1, 1),
        ]
        .iter()
        .filter(|n| lookup.contains(&(roll + *n)))
        .count();

        if count < 4 {
            total += 1;
        }
    }
    total
}

fn parse_input(input: &str) -> HashSet<IVec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '@' {
                    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
                    Some(IVec2::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(
            "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.",
        );
        assert_eq!(result, 13);
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
