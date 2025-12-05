use std::collections::VecDeque;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> usize {
    let mut fresh_ranges = Vec::with_capacity(256);
    for line in input.trim().lines() {
        if line.is_empty() {
            break;
        } else if let Some((low, high)) = line.split_once('-') {
            let low = low.parse::<u64>().unwrap();
            let high = high.parse::<u64>().unwrap();
            fresh_ranges.push(low..=high);
        } else {
            unreachable!()
        }
    }

    fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged_ranges = Vec::new();
    let mut working_ranges = VecDeque::from(fresh_ranges);
    while let Some(mut active_range) = working_ranges.pop_front() {
        let len = working_ranges.len();

        let mut modified = false;
        for _ in 0..len {
            if let Some(other_range) = working_ranges.pop_front() {
                if active_range.contains(other_range.start())
                    || active_range.contains(other_range.end())
                {
                    if active_range.end() != other_range.end()
                        || active_range.start() != other_range.start()
                    {
                        let low = *active_range.start().min(other_range.start());
                        let high = *active_range.end().max(other_range.end());
                        active_range = low..=high;
                    }
                    modified = true;
                } else {
                    working_ranges.push_back(other_range);
                }
            } else {
                unreachable!()
            }
        }

        if modified {
            working_ranges.push_back(active_range);
        } else {
            merged_ranges.push(active_range.clone());
        }
    }

    merged_ranges
        .into_iter()
        .map(std::iter::Iterator::count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32");
        assert_eq!(result, 14);
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
