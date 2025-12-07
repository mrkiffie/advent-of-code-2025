use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IVec2 {
    x: i32,
    y: i32,
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn solve(input: &str) -> usize {
    let mut lookup = HashMap::<IVec2, bool>::with_capacity(1792);
    let mut start = IVec2 { x: 0, y: 0 };
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = IVec2 {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                '^' => {
                    lookup.insert(
                        IVec2 {
                            x: x as i32,
                            y: y as i32,
                        },
                        false,
                    );
                }
                _ => {}
            }
        }
    }

    let max = lookup.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let mut visited = HashSet::<IVec2>::with_capacity(3584);
    let mut queue = VecDeque::with_capacity(128);
    queue.push_back(start);
    while let Some(mut beam) = queue.pop_front() {
        if visited.contains(&beam) {
            continue;
        }
        visited.insert(beam);
        while beam.y <= max {
            beam.y += 1;
            if let Some(splitter) = lookup.get_mut(&beam) {
                *splitter = true;
                let left = IVec2 {
                    x: beam.x - 1,
                    y: beam.y,
                };
                if !visited.contains(&left) {
                    queue.push_back(left);
                }
                let right = IVec2 {
                    x: beam.x + 1,
                    y: beam.y,
                };
                if !visited.contains(&right) {
                    queue.push_back(right);
                }

                break;
            }
        }
    }
    lookup.values().filter(|x| **x).count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(
            ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............",
        );
        assert_eq!(result, 21);
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
