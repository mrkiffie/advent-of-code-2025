const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

const INPUT_ROW_WIDTH: usize = 141;

fn solve(input: &[u8]) -> usize {
    let mut previous = [0usize; INPUT_ROW_WIDTH];
    let mut current = [0usize; INPUT_ROW_WIDTH];
    let mut i = 0;
    let mut count = 0;
    for c in input {
        match c {
            b'\n' => {
                i = 0;
                previous.copy_from_slice(&current);
            }
            b'.' => {
                i += 1;
            }
            b'S' => {
                current[i] = 1;
                i += 1;
            }
            b'^' => {
                debug_assert!(i > 0);
                if previous[i] != 0 {
                    count += 1;
                }
                current[i] = 0;
                current[i - 1] += previous[i];
                current[i + 1] += previous[i];
                i += 1;
            }
            _ => unreachable!(),
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(
            b".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............",
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
