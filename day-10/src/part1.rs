use std::collections::{HashMap, VecDeque, hash_map::Entry};

const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BitFlags(u16);

impl BitFlags {
    fn toggle(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

#[derive(Debug)]
struct Machine {
    /// The target state of the lights
    lights: BitFlags,
    buttons: Vec<BitFlags>,
}

impl Machine {
    fn solve(self) -> usize {
        if self.buttons.contains(&self.lights) {
            return 1;
        }

        let mut cache = HashMap::<BitFlags, usize>::new();

        let mut queue: VecDeque<(BitFlags, usize)> = self
            .buttons
            .iter()
            .copied()
            .map(|bitflag| (bitflag, 1))
            .collect();

        while let Some((state, steps)) = queue.pop_front() {
            if state == self.lights {
                return steps;
            }
            let new_steps = steps + 1;

            for button in &self.buttons {
                let new_state = state.toggle(*button);
                if let Entry::Vacant(entry) = cache.entry(new_state) {
                    entry.insert(new_steps);
                    queue.push_back((new_state, new_steps));
                }
            }
        }

        unreachable!()
    }
}

fn solve(input: &[u8]) -> usize {
    let mut machines = Vec::with_capacity(178);
    let mut value = 0;
    let mut counter = 0;
    let mut lights = BitFlags(0);
    let mut buttons = Vec::new();
    for b in input {
        match b {
            b'[' => {
                counter = 0;
            }
            b']' => {
                lights = BitFlags(value);
            }
            b'.' => {
                counter += 1;
            }
            b'#' => {
                value |= 1 << counter;
                counter += 1;
            }
            b'(' => {
                value = 0;
            }
            b')' => {
                buttons.push(BitFlags(value));
            }
            b'0'..=b'9' => {
                value |= 1 << usize::from(b - b'0');
            }
            b'\n' => {
                machines.push(Machine { lights, buttons });
                buttons = vec![];
                lights = BitFlags(0);
                counter = 0;
                value = 0;
            }
            b' ' | b',' | b'{' | b'}' => {}
            _ => {
                unreachable!()
            }
        }
    }
    // handles the case of trailing empty lines
    if !buttons.is_empty() {
        machines.push(Machine { lights, buttons });
    }

    machines.into_iter().map(Machine::solve).sum()
}

#[cfg(test)]
mod tests {
    use crate::part1::BitFlags;

    use super::solve;

    #[test]
    fn example_1() {
        let result = solve(b"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = solve(b"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let result = solve(b"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_1() {
        let result = solve(b"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_bitflags_toggling() {
        assert_eq!(BitFlags(0b0110).toggle(BitFlags(0b0011)), BitFlags(0b0101));
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
