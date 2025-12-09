const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

#[derive(Debug, Copy, Clone)]
struct IVec2 {
    x: i32,
    y: i32,
}

impl IVec2 {
    fn area(self, rhs: IVec2) -> i64 {
        i64::from((self.x - rhs.x).abs() + 1) * i64::from((self.y - rhs.y).abs() + 1)
    }
}

fn solve(input: &[u8]) -> i64 {
    let mut coordinates = Vec::new();

    let mut value: i32 = 0;
    let mut x = 0;

    for b in input {
        match b {
            b'0'..=b'9' => value = value * 10 + i32::from(b - b'0'),
            b',' => {
                x = value;
                value = 0;
            }
            b'\n' => {
                coordinates.push(IVec2 { x, y: value });
                value = 0;
                x = 0;
            }
            _ => {
                unreachable!();
            }
        }
    }

    let mut max_area = 0;

    for (i, a) in coordinates.iter().enumerate() {
        for b in coordinates.iter().skip(i + 1) {
            if a.area(*b) > max_area {
                max_area = a.area(*b);
            }
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(b"7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n");
        assert_eq!(result, 50);
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
