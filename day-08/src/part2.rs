use std::collections::HashMap;

use glam::I64Vec3;

const INPUT: &str = include_str!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &str) -> i64 {
    let nodes = parse_input(input);
    let node_count = nodes.len();

    let mut distances = HashMap::<Tuple, i64>::with_capacity(node_count * node_count);

    for row in &nodes {
        for col in &nodes {
            if col == row {
                continue;
            }

            distances.insert(Tuple::new(*col, *row), row.distance_squared(*col));
        }
    }

    let mut sorted: Vec<(Tuple, i64)> = distances.into_iter().collect();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));
    let mut node_to_circuit_lookup = HashMap::<I64Vec3, usize>::new();
    let mut circuits = HashMap::<usize, Vec<I64Vec3>>::new();
    let mut circuit_id = 0;

    for (Tuple(a, b), _) in &sorted {
        let circuit_a = node_to_circuit_lookup.get(a).copied();
        let circuit_b = node_to_circuit_lookup.get(b).copied();
        match (circuit_a, circuit_b) {
            (None, None) => {
                node_to_circuit_lookup.insert(*a, circuit_id);
                node_to_circuit_lookup.insert(*b, circuit_id);
                circuits.insert(circuit_id, vec![*a, *b]);
                circuit_id += 1;
            }

            (None, Some(circuit_id)) => {
                node_to_circuit_lookup.insert(*a, circuit_id);
                circuits.entry(circuit_id).and_modify(|nodes| {
                    nodes.push(*a);
                });
            }
            (Some(circuit_id), None) => {
                node_to_circuit_lookup.insert(*b, circuit_id);
                circuits.entry(circuit_id).and_modify(|nodes| {
                    nodes.push(*b);
                });
            }

            (Some(circuit_id_a), Some(circuit_id_b)) => {
                if circuit_a != circuit_b {
                    // merge circuit b in to circuit a
                    let to_move = circuits.get(&circuit_id_b).cloned().unwrap();

                    circuits.entry(circuit_id_a).and_modify(|nodes| {
                        nodes.extend_from_slice(&to_move);
                    });
                    for node in to_move {
                        node_to_circuit_lookup
                            .entry(node)
                            .and_modify(|id| *id = circuit_id_a);
                    }
                    circuits.remove(&circuit_id_b);
                }
            }
        }
        if circuits.len() == 1
            && let Some(c) = circuits.values().next()
            && c.len() == node_count
        {
            return a.x * b.x;
        }
    }
    unreachable!()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tuple(I64Vec3, I64Vec3);

impl Tuple {
    fn new(a: I64Vec3, b: I64Vec3) -> Self {
        match a.x.cmp(&b.x) {
            std::cmp::Ordering::Less => Self(a, b),
            std::cmp::Ordering::Greater => Self(b, a),
            std::cmp::Ordering::Equal => match a.y.cmp(&b.y) {
                std::cmp::Ordering::Less => Self(a, b),
                std::cmp::Ordering::Greater => Self(b, a),
                std::cmp::Ordering::Equal => match a.z.cmp(&b.z) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => Self(a, b),
                    std::cmp::Ordering::Greater => Self(b, a),
                },
            },
        }
    }
}

fn parse_line(input: &str) -> I64Vec3 {
    let mut nums = input.splitn(3, ',').filter_map(|i| i.parse::<i64>().ok());
    let x = nums.next().unwrap();
    let y = nums.next().unwrap();
    let z = nums.next().unwrap();
    I64Vec3::new(x, y, z)
}

fn parse_input(input: &str) -> Vec<I64Vec3> {
    input.lines().map(parse_line).collect()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(
            "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689",
        );
        assert_eq!(result, 25272);
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
