use std::collections::VecDeque;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    let mut inventory = Vec::with_capacity(1024);
    let mut fresh_ranges = Vec::with_capacity(256);
    let mut blank_lines = false;
    for line in input.trim().lines() {
        if line.is_empty() {
            blank_lines = true;
        } else if blank_lines {
            inventory.push(line.parse::<u64>().unwrap());
        } else if let Some((low, high)) = line.split_once('-') {
            let low = low.parse::<u64>().unwrap();
            let high = high.parse::<u64>().unwrap();
            fresh_ranges.push(low..=high);
        } else {
            unreachable!()
        }
    }

    inventory.sort_unstable();
    fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut fresh_ranges = VecDeque::from(fresh_ranges);
    let mut active_range = fresh_ranges.pop_front().unwrap();
    let mut count = 0;
    for id in inventory {
        while id > *active_range.end() {
            if let Some(next_range) = fresh_ranges.pop_front() {
                active_range = next_range;
            } else {
                return count;
            }
        }
        if id < *active_range.start() {
            continue;
        }
        if id <= *active_range.end() {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32");
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
