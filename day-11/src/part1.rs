use std::collections::{HashMap, VecDeque};

const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &[u8]) -> i32 {
    type Id = [u8; 3];
    assert_eq!(input.last(), Some(&b'\n'));

    let mut map: HashMap<Id, Vec<Id>> = HashMap::with_capacity(768);

    let mut key = [0; 3];
    let mut val = [0; 3];
    let mut i = 0;

    for b in input {
        match b {
            b'a'..=b'z' => {
                val[i] = *b;
                i += 1;
            }
            b':' => {
                key = val;
                i = 0;
            }
            b'\n' | b' ' => {
                if key != val {
                    map.entry(key)
                        .and_modify(|entry| entry.push(val))
                        .or_insert(vec![val]);
                }
                i = 0;
            }
            _ => unreachable!(),
        }
    }

    let mut queue = VecDeque::with_capacity(768);
    queue.push_back(b"you");

    let mut count = 0;
    while let Some(key) = queue.pop_front() {
        if let Some(branches) = map.get(key) {
            for branch in branches {
                if branch == b"out" {
                    count += 1;
                } else {
                    queue.push_back(branch);
                }
            }
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
            b"aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out\n",
        );
        assert_eq!(result, 5);
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
